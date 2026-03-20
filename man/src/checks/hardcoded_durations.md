# `src/checks/hardcoded_durations.rs`

## `pub fn check(ctx: &FileContext, lines: &[&str], issues: &mut Vec<Issue>)`
*Line 20 · fn*

Scan for hardcoded Duration literals — emit issues for Duration::from_secs/millis/nanos/etc with non-zero arguments.

---



---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=2" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
