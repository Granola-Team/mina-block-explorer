function renderBoxAndWhiskerPlot(data, myChart) {
  let xAxis = Object.keys(data);
  let option;

  myChart.hideLoading();

  option = {
    title: {
      ...TITLE_DEFAULT,
      text: `Fee Transfers`,
    },
    color: [...CHART_COLORS],
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    grid: { ...GRID_DEFAULT },
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
      ...X_AXIS_DEFAULT,
      type: "category",
      name: "Global Slot",
      axisLabel: {
        ...X_AXIS_LABEL_DEFAULT,
        formatter: function (value) {
          return xAxis[value];
        },
      },
      splitLine: {
        ...GRID_LINES,
        show: false,
      },
    },
    yAxis: [
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Fee (MINA)",
        axisLabel: {
          ...Y_AXIS_AXIS_LABEL_DEFAULT,
          formatter: function (value) {
            return `${value}`;
          },
        },
      },
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Transfers Count",
        position: "right",
        axisLabel: {
          ...Y_AXIS_AXIS_LABEL_DEFAULT,
          formatter: function (value) {
            return `${value}`;
          },
        },
        splitLine: {
          ...GRID_LINES,
          show: false,
        },
      },
    ],
    series: [
      {
        name: "boxplot",
        type: "boxplot",
        datasetIndex: 1,
        yAxisIndex: 0,
        tooltip: {
          formatter: function (param) {
            return [
              "Slot: " +
                xAxis[param.name] +
                `-${+xAxis[param.name] + groupSize - 1}`,
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
        symbolSize: 8,
        datasetIndex: 2,
        yAxisIndex: 0,
        tooltip: {
          formatter: function (param) {
            return [
              "Slot: " +
                xAxis[param.name] +
                `-${+xAxis[param.name] + groupSize - 1}`,
              "Fee: " + param.data[1],
            ].join("<br/>");
          },
        },
      },
      {
        name: "transfers",
        type: "line",
        data: Object.values(data).map((fees, i) => ["" + i, fees.length]),
        yAxisIndex: 1,
        tooltip: {
          formatter: function (param) {
            return `Slot: ${xAxis[param.dataIndex]}-${+xAxis[param.dataIndex] + groupSize - 1}<br/>Transfers: ${param.value[1]}`;
          },
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

setTimeout(async () => {
  const blockLimit = getBlockLimit();

  let chartDom = document.getElementById("chart");
  window.addEventListener("resize", function () {
    boxAndWhiskerChart.resize();
  });
  let boxAndWhiskerChart = echarts.init(chartDom);

  boxAndWhiskerChart.showLoading({
    text: "Loading...", // Display text with the spinner
    color: "#E39844", // Spinner color
    zlevel: 0,
  });

  let response = await fetch(config.graphql_endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: `query MyQuery() {
        feetransfers(query: { canonical: true }, sortBy: BLOCKHEIGHT_DESC, limit: ${blockLimit}) {
          fee,
          blockStateHash {
            protocolState {
              consensusState {
                slotSinceGenesis
              }
            }
          }
        }
      }
    `,
    }),
  });

  let jsonResp = await response.json();
  let data = jsonResp.data.feetransfers.reduce((agg, record) => {
    let slot =
      record.blockStateHash.protocolState.consensusState.slotSinceGenesis;
    let key = slot - (slot % SLOT_GROUPING);
    let value = record.fee;
    if (!agg[key]) {
      agg[key] = [];
    }
    let parsedFloat = parseFloat(value / 1e9);
    if (parsedFloat < 700) {
      agg[key].push(parsedFloat);
    }
    return agg;
  }, {});

  renderBoxAndWhiskerPlot(data, boxAndWhiskerChart);
}, 1000);
