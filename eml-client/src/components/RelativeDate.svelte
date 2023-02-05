<script>
  export let date

  const hoursDiff = (date) => {
    const now = new Date()
    return (now - date) / (60 * 60 * 1000)
  }

  const formatDate = (date) => {
    const relativeDate = new Intl.RelativeTimeFormat("en-GB", {
      numeric: "auto",
    })

    const weekdayDate = new Intl.DateTimeFormat("en-GB", {
      weekday: "long",
    })

    const fullDate = new Intl.DateTimeFormat("en-GB", {
      weekday: "short",
      day: "numeric",
      month: "short",
      year: "numeric",
    })

    if (!date) {
      console.error(`Cannot format date ${date}`)
      return "?"
    }

    if (hoursDiff(date) < 1.5) {
      return relativeDate.format(-(hoursDiff(date) * 60).toFixed(0), "minutes")
    }

    if (hoursDiff(date) < 20) {
      return relativeDate.format(-hoursDiff(date)
        .toFixed(0), "hours")
    }

    if (hoursDiff(date) < 7 * 24) {
      return weekdayDate.format(date)
    }

    return fullDate.format(date)
  }

  $: formattedDate = date ? formatDate(date) : null
  const msHour = 1000 * 60 * 60
  setInterval(
    () => (formattedDate = formatDate(date)),
    (hoursDiff(date) * msHour) / 1.5 / 60,
  )
</script>

<span>{formattedDate}</span>
