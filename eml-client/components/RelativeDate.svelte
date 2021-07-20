<script>
  export let date;

  const formatDate = (date) => {
    const relativeDate = new Intl.RelativeTimeFormat("en-GB", {
      numeric: "auto",
    });

    const weekdayDate = new Intl.DateTimeFormat("en-GB", { weekday: "long" });

    const fullDate = new Intl.DateTimeFormat("en-GB", {
      weekday: "short",
      day: "numeric",
      month: "short",
      year: "numeric",
    });

    if (!date) {
      console.error(`Cannot format date ${date}`);
      return "?";
    }

    const now = new Date();
    const hoursDiff = (now - date) / (60 * 60 * 1000);

    if (hoursDiff < 1.5)
      return relativeDate.format(-(hoursDiff * 60).toFixed(0), "minutes");

    if (hoursDiff < 20)
      return relativeDate.format(-hoursDiff.toFixed(0), "hours");

    if (hoursDiff < 7 * 24) return weekdayDate.format(date);

    return fullDate.format(date);
  };

</script>

<span>{formatDate(date)}</span>
