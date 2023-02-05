<script context="module">
  import ICAL from "ical.js"

  export const parseVCalEvents = (content) => {
    const jCal = ICAL.parse(content)
    const comp = new ICAL.Component(jCal)
    return comp.getAllSubcomponents("vevent")
      .map((e) => new ICAL.Event(e))
  }
</script>

<script>
  export let vcal

  export let full = false
  export let primaryEventSummary = null

  $: sortedEvents = parseVCalEvents(vcal)
    .sort((a, b) => a.summary == primaryEventSummary
      ? -1
      : b.summary == primaryEventSummary
        ? 1
        : 0,
    )

  const formatDateRange = (start, end) => {
    start = start.toJSDate()
    end = end.toJSDate()

    const formatDate = (date, long = true) => date.toLocaleDateString(undefined, {
      day: "numeric",
      month: long ? "long" : "numeric",
      weekday: long ? "short" : undefined,
      year: "numeric",
    })

    let startDate = formatDate(start)
    let endDate = formatDate(end)

    const formatTime = (date, tz = true) => date.toLocaleTimeString(undefined, {
      timeZoneName: tz ? "short" : undefined,
    })

    const showBothTimezones =      start.getTimezoneOffset() != end.getTimezoneOffset()
    const startTime = formatTime(start, showBothTimezones)
    const endTime = formatTime(end, true)

    if (startDate == endDate) {
      return startTime == endTime
        ? `${startDate} @ ${startTime}`
        : `${startDate} ${startTime}-${endTime}`
    }

    startDate = formatDate(start, false)
    endDate = formatDate(end, false)
    return `${startDate} ${startTime} - ${endDate} ${endTime}`
  }
</script>

{#each sortedEvents as event}
  {#if event.summary != primaryEventSummary}
    <h3>{event.summary}</h3>
  {/if}

  <h4>{formatDateRange(event.startDate, event.endDate)}</h4>

  <h4>@ {event.location}</h4>

  {#if full && event.description}
    {#each event.description.split(/\n\n/) as para}
      <p>{para}</p>
    {/each}
  {/if}
{/each}
