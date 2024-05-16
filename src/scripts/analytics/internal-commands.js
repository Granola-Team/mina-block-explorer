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

  let xAxis = Object.keys(data);

  let chartDom = document.getElementById("chart");
  window.addEventListener("resize", function () {
    myChart.resize();
  });
  let myChart = echarts.init(chartDom);
  let option;

  option = {
    title: {
      text: "Fee Transfers in the last 24 hours",
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
      {
        fromDatasetIndex: 1,
        fromTransformResult: 1,
      },
    ],
    xAxis: {
      type: "category",
      name: "Hour",
      axisLabel: {
        rotate: 45,
      },
      axisLabel: {
        formatter: function (value) {
          return xAxis[value];
        },
      },
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
        tooltip: {
          formatter: function (param) {
            return [
              "Date: " + xAxis[param.name],
              "upper: " + param.data[5],
              "Q3: " + param.data[4],
              "median: " + param.data[3],
              "Q1: " + param.data[2],
              "lower: " + param.data[1],
            ].join("<br/>");
          },
        },
      },
      {
        name: "boxplot",
        type: "scatter",
        datasetIndex: 2,
        tooltip: {
          formatter: function (param) {
            return ["Date: " + xAxis[param.name], "Fee: " + param.data[1]].join(
              "<br/>",
            );
          },
        },
      },
    ],
  };

  option && myChart.setOption(option);
}, 1000);