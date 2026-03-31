---
name: auto-test-quality
description: "Test quality discipline: meaningful assertions, mock boundaries, tautological test detection, and the specific test anti-patterns Claude defaults to. Corrects tests that pass but protect nothing — over-mocking, asserting on existence instead of correctness, and snapshot-testing implementation details. Use when writing tests, reviewing test quality, or debugging false-passing tests. Triggers: test, test quality, assertion, mock, stub, spy, snapshot, tautological, test coverage, false positive, test value, meaningful test, vitest, jest, pytest, proptest."
---

# Test Quality — What Claude Gets Wrong

Your tests pass. They also protect nothing. You generate tests that *look like* tests — correct syntax, proper structure, green checkmarks — but don't actually verify behavior. A tautological test is worse than no test: it creates false confidence.

## The Tautology Test

> "If I replaced the function body with `return null`, would this test fail?"

If no, the test is tautological. Delete it and write a real one.

## Anti-Patterns You Default To

| Anti-pattern | Example | Fix |
|---|---|---|
| Asserting existence, not value | `expect(result).toBeDefined()` | `expect(result).toEqual({ id: 1, name: "Alice" })` |
| Mock returns the expected value | `mock.mockReturnValue(X); expect(fn()).toBe(X)` | Test real code — mock only external boundaries |
| Asserting type, not behavior | `assert isinstance(result, dict)` | `assert result["status"] == "completed"` |
| "Doesn't throw" as success | `expect(() => fn()).not.toThrow()` | Assert the return value IS correct |
| Snapshot of implementation | `expect(component).toMatchSnapshot()` on every test | Snapshot only stable public output, assert behavior explicitly |
| Mock the thing you're testing | `vi.spyOn(service, 'calculate').mockReturnValue(42)` then test `service.calculate()` | Don't mock the subject — mock its dependencies |
| Identical test names | Three tests named "should work correctly" | Name by specific behavior: "returns 0 for empty input" |
| No negative tests | Only test happy path | Test that invalid input produces errors, not just that valid input works |
| Copied expected from output | Run code, copy output, paste as expected | Derive expected independently from the specification |

## The Mock Boundary Rule

**Mock at the system boundary. Never mock internal logic.**

| Mock This | Don't Mock This |
|-----------|----------------|
| Database queries | Business logic functions |
| HTTP requests to external APIs | Internal service methods |
| File system I/O | Pure computation |
| System clock (`Date.now()`) | Your own utility functions |
| Random number generation | Data transformation |

```typescript
// You write — mocking the subject:
test('calculates total', () => {
  const mockCalc = vi.spyOn(pricing, 'calculateTotal').mockReturnValue(150);
  expect(pricing.calculateTotal(items)).toBe(150);
  // This tests that vi.spyOn works, not that calculateTotal works
});

// Senior writes — mocking the boundary:
test('calculates total with tax', () => {
  vi.spyOn(taxService, 'getRate').mockReturnValue(0.0925); // External boundary
  const total = pricing.calculateTotal([{ price: 100, qty: 1 }]);
  expect(total).toBe(109.25); // Real calculation with mocked tax rate
});
```

## Meaningful Assertions

```python
# You write:
def test_process_report():
    result = process_report(sample_data)
    assert result is not None
    assert len(result) > 0
    assert isinstance(result[0], dict)
    # Proves nothing about correctness

# Senior writes:
def test_process_report_aggregates_by_sentinel():
    result = process_report([
        {"sentinel": "CHRONOS", "value": 100},
        {"sentinel": "CHRONOS", "value": 200},
        {"sentinel": "LEVIATHAN", "value": 50},
    ])
    assert result == {
        "CHRONOS": {"count": 2, "total": 300, "avg": 150.0},
        "LEVIATHAN": {"count": 1, "total": 50, "avg": 50.0},
    }
```

## Test Naming Convention

Name tests by behavior, not by method:

```
BAD:  test_calculate, test_process, test_handle_request
GOOD: test_returns_zero_for_empty_input
GOOD: test_rejects_negative_amounts
GOOD: test_aggregates_duplicate_sentinels
GOOD: test_retries_on_transient_failure
```

The name should tell you what's broken when the test fails.

## The Four-Test Minimum

For any non-trivial function, you need at least:

1. **Happy path** — normal input produces correct output
2. **Edge case** — empty, zero, boundary input handled correctly
3. **Error case** — invalid input produces the right error
4. **Specific behavior** — the one thing this function does differently from similar functions

If you can only write one test, write the specific-behavior test. It has the highest signal.

## Property-Based Testing

When a function has mathematical properties, test the property, not individual cases:

```rust
// You write 5 example-based tests. Senior writes:
proptest! {
    #[test]
    fn consensus_is_deterministic(votes in prop::collection::vec(vote_strategy(), 3)) {
        let result1 = evaluate_consensus(&votes);
        let result2 = evaluate_consensus(&votes);
        assert_eq!(result1, result2); // Same input → same output, always
    }
}
```

Properties to test: idempotency, commutativity, round-trip (encode/decode), invariant preservation.
