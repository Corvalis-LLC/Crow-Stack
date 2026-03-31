# Triumvirate Templates

## Synthesis Template

After all three subagents return, synthesize their arguments:

```markdown
## Triumvirate Review Complete

### The Advocate's Case
{advocate_summary}

### The Analyst's Assessment
{analyst_summary}

### The Critic's Concerns
{critic_summary}

---

## Synthesis

### Points of Agreement
- [What all three perspectives agree on]

### Key Tensions
- [Where perspectives conflict and why]

### Strongest Arguments
From Advocate: [strongest pro argument]
From Analyst: [most important tradeoff]
From Critic: [most valid concern]
```

## Amended Plan Template

Revise the original plan incorporating:
- Valid concerns raised by the Critic (with mitigations)
- Alternative approaches suggested by the Analyst (if better)
- Enhancements suggested by the Advocate (if valuable)

```markdown
## Amended Plan

### Original Plan
{original_plan}

### Revisions Based on Triumvirate Review

#### Incorporated from Advocate
- [Enhancement added]

#### Incorporated from Analyst
- [Alternative approach adopted]
- [Metric added]

#### Incorporated from Critic
- [Risk mitigation added]
- [Edge case handling added]

### Final Plan
{revised_plan_text}
```

## Presentation Template

Present the amended plan and options directly without using AskUserQuestion:

```markdown
## Triumvirate Review Complete

[Synthesis and amended plan presented above]

### Options
- **Re-debate**: Reply "/triumvirate" to have reviewers debate the amended plan
- **Approve**: Reply "approve" or "proceed" to begin implementation
- **Modify**: Reply with your changes
- **Reject**: Reply "reject" to start over

What would you like to do?
```
