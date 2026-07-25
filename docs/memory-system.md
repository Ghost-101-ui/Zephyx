# Memory System

The Memory System gives Zephyx the ability to learn from past assessments and apply that knowledge in future engagements.

---

## What Is the Memory System?

The Memory System stores **execution outcomes** from past assessments:
- Which tool flags were effective against specific service patterns
- How long certain operations took
- Which approaches succeeded or failed

This information is used by the Planner to generate more effective action plans in future sessions.

---

## Memory Record Structure

```rust
pub struct MemoryInsight {
    pub target_name: String,
    pub successful_tool: String,
    pub effective_flags: String,
    pub execution_time_secs: u64,
    pub context_tags: Vec<String>,
}
```

---

## Commands

```bash
zpx memory list
```

**Output:**
```
Zephyx Long-Term Knowledge Memory:
  - Tool: nmap         | Flags: -sV -sC -p-              | Time: 45s
  - Tool: ffuf         | Flags: -w common.txt -mc 200,301 | Time: 30s
  - Tool: enum4linux   | Flags: -a                        | Time: 120s
```

---

## How Memory Is Built

1. When a task completes successfully, its execution record is stored
2. The record includes: tool name, flags used, target context tags, execution time
3. On future assessments, the Planner queries memory to seed initial command suggestions
4. Over time, memory builds a personalized library of effective techniques

---

## Memory vs Knowledge Packs

| Feature | Memory System | Knowledge Packs |
|---|---|---|
| Source | Your past sessions | Pre-built by community |
| Updates | Automatic (from your work) | Manual (pack updates) |
| Scope | Your personal patterns | General security knowledge |
| Location | `~/.zephyx/database/` | `~/.zephyx/knowledge/` |
