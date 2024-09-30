function renderTransferCountPlot(data, myChart) {
  let xAxis = Object.keys(data);
  let option;

  myChart.hideLoading();

  option = {
    title: {
      ...TITLE_DEFAULT,
      text: `Fee Transfer Count`,
    },
    color: [...CHART_COLORS],
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    grid: { ...GRID_DEFAULT },
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
    },
    yAxis: {
      ...Y_AXIS_DEFAULT,
      type: "value",
      name: "Transfers Count",
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        formatter: function (value) {
          return `${value}`;
        },
      },
    },
    series: [
      {
        name: "transfers",
        type: "bar",
        data: Object.values(data).map((fees, i) => ["" + i, fees.length]),
        tooltip: {
          formatter: function (param) {
            return [
              [
                "Slot",
                xAxis[param.name] +
                  `-${+xAxis[param.name] + SLOT_GROUPING - 1}`,
              ],
              ["count", param.data[1]],
            ]
              .map(([a, b]) => `<strong>${a}</strong>: ${b}`)
              .join("</br>");
          },
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

function renderBoxAndWhiskerPlot(data, myChart) {
  let xAxis = Object.keys(data);
  let option;

  myChart.hideLoading();

  option = {
    title: {
      ...TITLE_DEFAULT,
      text: `Fee Transfer Spread`,
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
              [
                "Slot",
                xAxis[param.name] +
                  `-${+xAxis[param.name] + SLOT_GROUPING - 1}`,
              ],
              ["max", param.data[5]],
              ["Q3", param.data[4]],
              ["median", param.data[3]],
              ["Q1", param.data[2]],
              ["min", param.data[1]],
            ]
              .map(([a, b]) => `<strong>${a}</strong>: ${b}`)
              .join("</br>");
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
              [
                "Slot",
                xAxis[param.name] +
                  `-${+xAxis[param.name] + SLOT_GROUPING - 1}`,
              ],
              ["Fee", param.data[1]],
            ]
              .map(([a, b]) => `<strong>${a}</strong>: ${b}`)
              .join("</br>");
          },
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

setTimeout(async () => {
  const blockLimit = getBlockLimit();

  let feeSpreadDom = document.getElementById("fee-spread");
  let feeCountsDom = document.getElementById("transfer-count");
  window.addEventListener("resize", function () {
    boxAndWhiskerChart.resize();
    barPlot.resize();
  });
  let boxAndWhiskerChart = echarts.init(feeSpreadDom);
  let barPlot = echarts.init(feeCountsDom);

  boxAndWhiskerChart.showLoading({
    text: "Loading...", // Display text with the spinner
    color: "#E39844", // Spinner color
    zlevel: 0,
  });

  barPlot.showLoading({
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
  renderTransferCountPlot(data, barPlot);
}, 1000);
