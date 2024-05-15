setTimeout(async () => {
  let response = await fetch(config.graphql_endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: `query MyQuery($date: DateTime!) {
        feetransfers(query: {canonical: true, dateTime_gte: $date}, sortBy: BLOCKHEIGHT_ASC, limit: 1000000) {
          fee
          dateTime
        }
      } 
    `,
      variables: {
        date: new Date(
          new Date().getTime() - 24 * 60 * 60 * 1000,
        ).toISOString(),
      },
    }),
  });

  let jsonResp = await response.json();
  let data = jsonResp.data.feetransfers.reduce((agg, record) => {
    const date = new Date(record.dateTime);
    const key =
      date.getUTCFullYear() +
      "-" +
      String(date.getUTCMonth() + 1).padStart(2, "0") + // Month is zero-indexed, add one
      "-" +
      String(date.getUTCDate()).padStart(2, "0") +
      " " +
      String(date.getUTCHours()).padStart(2, "0");
    let value = record.fee;
    if (!agg[key]) {
      agg[key] = [];
    }
    agg[key].push(parseFloat(value / 1e9));
    return agg;
  }, {});

  console.log(data);

  let chartDom = document.getElementById("chart");
  let myChart = echarts.init(chartDom);
  let option;

  option = {
    title: {
      text: "Fee Transfers",
      left: "center",
    },
    tooltip: {
      trigger: "item",
      axisPointer: {
        type: "shadow",
      },
    },
    dataset: [
      {
        source: Object.entries(data).map(([_, fees]) => [...fees]),
      },
      {
        fromDatasetIndex: 0,
        transform: {
          type: "boxplot",
        },
      },
    ],
    xAxis: {
      type: "category",
      name: "Hour",
    },
    yAxis: {
      type: "value",
      name: "Fee",
    },
    series: [
      {
        name: "boxplot",
        type: "boxplot",
        datasetIndex: 1,
      },
    ],
  };

  option && myChart.setOption(option);
}, 1000);
