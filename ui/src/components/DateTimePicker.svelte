<script lang="ts">
  interface Props {
    value: string;
    onchange: (value: string) => void;
  }

  let { value, onchange }: Props = $props();

  const DAYS = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
  const MONTHS = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
  ];

  let isOpen = $state(false);

  // Parse initial value or default to now
  function parseValue(v: string): { year: number; month: number; day: number; hour: number; minute: number } {
    if (v) {
      const d = new Date(v);
      if (!isNaN(d.getTime())) {
        return {
          year: d.getFullYear(),
          month: d.getMonth(),
          day: d.getDate(),
          hour: d.getHours(),
          minute: d.getMinutes(),
        };
      }
    }
    const now = new Date();
    now.setHours(now.getHours() + 1, 0, 0, 0);
    return {
      year: now.getFullYear(),
      month: now.getMonth(),
      day: now.getDate(),
      hour: now.getHours(),
      minute: now.getMinutes(),
    };
  }

  const initial = parseValue(value);
  let viewYear = $state(initial.year);
  let viewMonth = $state(initial.month);
  let selectedDay = $state(initial.day);
  let selectedHour = $state(initial.hour);
  let selectedMinute = $state(initial.minute);

  // Emit initial value if none was provided
  if (!value) {
    // Use queueMicrotask to avoid emitting during init
    queueMicrotask(() => emitValue());
  }

  // 12h display
  let displayHour = $derived(selectedHour === 0 ? 12 : selectedHour > 12 ? selectedHour - 12 : selectedHour);
  let isPM = $derived(selectedHour >= 12);

  function emitValue() {
    const d = new Date(viewYear, viewMonth, selectedDay, selectedHour, selectedMinute);
    // Format as datetime-local value: YYYY-MM-DDTHH:MM
    const yyyy = d.getFullYear();
    const mm = String(d.getMonth() + 1).padStart(2, "0");
    const dd = String(d.getDate()).padStart(2, "0");
    const hh = String(d.getHours()).padStart(2, "0");
    const min = String(d.getMinutes()).padStart(2, "0");
    onchange(`${yyyy}-${mm}-${dd}T${hh}:${min}`);
  }

  function selectDay(day: number) {
    selectedDay = day;
    emitValue();
  }

  function prevMonth() {
    if (viewMonth === 0) {
      viewMonth = 11;
      viewYear--;
    } else {
      viewMonth--;
    }
  }

  function nextMonth() {
    if (viewMonth === 11) {
      viewMonth = 0;
      viewYear++;
    } else {
      viewMonth++;
    }
  }

  function adjustHour(delta: number) {
    selectedHour = ((selectedHour + delta) + 24) % 24;
    emitValue();
  }

  function adjustMinute(delta: number) {
    selectedMinute = ((selectedMinute + delta) + 60) % 60;
    emitValue();
  }

  function toggleAmPm() {
    selectedHour = (selectedHour + 12) % 24;
    emitValue();
  }

  // Calendar grid
  function getCalendarDays(year: number, month: number): (number | null)[] {
    const firstDay = new Date(year, month, 1).getDay();
    // Convert Sunday=0 to Monday-start: Mon=0, Tue=1, ..., Sun=6
    const startOffset = firstDay === 0 ? 6 : firstDay - 1;
    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const daysInPrevMonth = new Date(year, month, 0).getDate();

    const cells: (number | null)[] = [];

    // Previous month trailing days
    for (let i = startOffset - 1; i >= 0; i--) {
      cells.push(-(daysInPrevMonth - i)); // Negative = prev month
    }

    // Current month
    for (let d = 1; d <= daysInMonth; d++) {
      cells.push(d);
    }

    // Next month leading days
    const remaining = 42 - cells.length; // 6 rows
    for (let d = 1; d <= remaining; d++) {
      cells.push(null); // null = next month
    }

    // Trim last row if empty
    while (cells.length > 35 && cells.slice(-7).every((c) => c === null || (typeof c === "number" && c < 0))) {
      // Keep at least 5 rows
    }

    return cells;
  }

  let calendarDays = $derived(getCalendarDays(viewYear, viewMonth));

  let today = $derived(() => {
    const now = new Date();
    return { year: now.getFullYear(), month: now.getMonth(), day: now.getDate() };
  });

  function isToday(day: number): boolean {
    const t = today();
    return day === t.day && viewMonth === t.month && viewYear === t.year;
  }

  function isSelected(day: number): boolean {
    return day === selectedDay;
  }

  // Formatted display
  let displayText = $derived(() => {
    if (!value) return "Pick date & time";
    const d = new Date(value);
    if (isNaN(d.getTime())) return "Pick date & time";
    const mon = MONTHS[d.getMonth()].slice(0, 3);
    const day = d.getDate();
    const year = d.getFullYear();
    const h = d.getHours();
    const m = String(d.getMinutes()).padStart(2, "0");
    const dh = h === 0 ? 12 : h > 12 ? h - 12 : h;
    const ampm = h >= 12 ? "PM" : "AM";
    return `${mon} ${day}, ${year}  ${dh}:${m} ${ampm}`;
  });

  // Close on outside click
  function handleClickOutside(e: MouseEvent) {
    const target = e.target as HTMLElement;
    if (!target.closest(".datetime-picker")) {
      isOpen = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="datetime-picker">
  <button class="picker-trigger" onclick={() => (isOpen = !isOpen)} type="button">
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/>
    </svg>
    <span>{displayText()}</span>
  </button>

  {#if isOpen}
    <div class="dropdown">
      <!-- Calendar -->
      <div class="calendar">
        <div class="cal-header">
          <span class="cal-title">{MONTHS[viewMonth].slice(0, 3)} {viewYear}</span>
          <div class="cal-nav">
            <button class="nav-btn" onclick={prevMonth} type="button">&lsaquo;</button>
            <button class="nav-btn" onclick={nextMonth} type="button">&rsaquo;</button>
          </div>
        </div>

        <div class="cal-grid">
          {#each DAYS as day}
            <span class="cal-day-label">{day}</span>
          {/each}

          {#each calendarDays as cell}
            {#if cell === null}
              <span class="cal-cell other"></span>
            {:else if cell < 0}
              <span class="cal-cell other">{Math.abs(cell)}</span>
            {:else}
              <button
                class="cal-cell"
                class:today={isToday(cell)}
                class:selected={isSelected(cell)}
                onclick={() => selectDay(cell)}
                type="button"
              >
                {cell}
              </button>
            {/if}
          {/each}
        </div>
      </div>

      <!-- Time picker -->
      <div class="time-picker">
        <div class="time-separator"></div>
        <div class="time-controls">
          <div class="time-col">
            <button class="time-btn" onclick={() => adjustHour(1)} type="button">
              <svg width="10" height="10" viewBox="0 0 10 10"><path d="M2 7 L5 3 L8 7" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
            </button>
            <span class="time-value">{String(displayHour).padStart(2, "0")}</span>
            <button class="time-btn" onclick={() => adjustHour(-1)} type="button">
              <svg width="10" height="10" viewBox="0 0 10 10"><path d="M2 3 L5 7 L8 3" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
            </button>
          </div>

          <span class="time-colon">:</span>

          <div class="time-col">
            <button class="time-btn" onclick={() => adjustMinute(5)} type="button">
              <svg width="10" height="10" viewBox="0 0 10 10"><path d="M2 7 L5 3 L8 7" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
            </button>
            <span class="time-value">{String(selectedMinute).padStart(2, "0")}</span>
            <button class="time-btn" onclick={() => adjustMinute(-5)} type="button">
              <svg width="10" height="10" viewBox="0 0 10 10"><path d="M2 3 L5 7 L8 3" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/></svg>
            </button>
          </div>

          <button class="ampm-btn" onclick={toggleAmPm} type="button">
            {isPM ? "PM" : "AM"}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .datetime-picker {
    position: relative;
  }

  .picker-trigger {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
    padding: 9px 14px;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--surface);
    color: var(--text);
    font-size: 13.5px;
    font-family: "DM Sans", system-ui, sans-serif;
    cursor: pointer;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
    text-align: left;
  }

  .picker-trigger:hover {
    border-color: var(--border-strong);
  }

  .picker-trigger:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-glow);
  }

  .picker-trigger svg {
    color: var(--text-tertiary);
    flex-shrink: 0;
  }

  .dropdown {
    position: absolute;
    bottom: calc(100% + 6px);
    left: 0;
    z-index: 100;
    background: var(--surface-raised);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius-lg);
    box-shadow: 0 -8px 32px rgba(0, 0, 0, 0.5), 0 -2px 8px rgba(0, 0, 0, 0.3);
    padding: 14px;
    min-width: 260px;
    animation: dropdown-in 0.15s ease;
  }

  @keyframes dropdown-in {
    from {
      opacity: 0;
      transform: translateY(4px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Calendar */
  .calendar {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .cal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .cal-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text);
    letter-spacing: 0.2px;
  }

  .cal-nav {
    display: flex;
    gap: 2px;
  }

  .nav-btn {
    width: 26px;
    height: 26px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 18px;
    cursor: pointer;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s ease;
    line-height: 1;
  }

  .nav-btn:hover {
    background: var(--surface);
    color: var(--text);
  }

  .cal-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 1px;
  }

  .cal-day-label {
    font-size: 10px;
    font-family: "DM Mono", monospace;
    color: var(--text-tertiary);
    text-align: center;
    padding: 4px 0;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .cal-cell {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12.5px;
    color: var(--text);
    border: none;
    background: transparent;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.12s ease;
    font-family: "DM Sans", system-ui, sans-serif;
    padding: 0;
  }

  .cal-cell:hover:not(.other):not(.selected) {
    background: var(--surface);
    color: var(--text);
  }

  .cal-cell.other {
    color: var(--text-tertiary);
    opacity: 0.4;
    cursor: default;
  }

  .cal-cell.today:not(.selected) {
    color: var(--accent);
    font-weight: 700;
  }

  .cal-cell.selected {
    background: var(--accent);
    color: white;
    font-weight: 600;
    box-shadow: 0 2px 8px var(--accent-glow);
  }

  /* Time picker */
  .time-picker {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .time-separator {
    height: 1px;
    background: var(--border);
    margin-top: 10px;
  }

  .time-controls {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
  }

  .time-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2px;
  }

  .time-btn {
    width: 28px;
    height: 20px;
    border: none;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.12s ease;
    padding: 0;
  }

  .time-btn:hover {
    background: var(--surface);
    color: var(--text);
  }

  .time-value {
    font-size: 20px;
    font-family: "DM Mono", monospace;
    font-weight: 600;
    color: var(--text);
    letter-spacing: 1px;
    min-width: 36px;
    text-align: center;
    line-height: 1;
    padding: 4px 0;
  }

  .time-colon {
    font-size: 20px;
    font-family: "DM Mono", monospace;
    font-weight: 600;
    color: var(--text-tertiary);
    line-height: 1;
    padding-bottom: 2px;
  }

  .ampm-btn {
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    color: var(--accent);
    font-size: 11px;
    font-family: "DM Mono", monospace;
    font-weight: 600;
    letter-spacing: 0.8px;
    cursor: pointer;
    transition: all 0.15s ease;
    margin-left: 6px;
  }

  .ampm-btn:hover {
    border-color: var(--accent);
    background: var(--accent-subtle);
  }
</style>
