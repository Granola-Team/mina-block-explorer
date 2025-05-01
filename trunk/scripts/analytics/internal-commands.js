function renderTransferCountPlot(data, myChart) {
  let transfer_counts = Object.entries(data);
  transfer_counts.sort(
    (a, b) => parseInt(a[0].split("-")[0]) - parseInt(b[0].split("-")[0]),
  );
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
      name: "Block Height",
      data: transfer_counts.map(([height]) => height),
      axisLabel: {
        ...X_AXIS_LABEL_DEFAULT,
        formatter: (v) => v.split("-")[0],
      },
    },
    yAxis: {
      ...Y_AXIS_DEFAULT,
      type: "value",
      name: "Transfers Count",
    },
    series: [
      {
        name: "transfers",
        type: "bar",
        data: transfer_counts.map(([_, fee]) => fee.length),
      },
    ],
  };

  option && myChart.setOption(option);
}

function renderBoxAndWhiskerPlotIT(fees, myChart) {
  let option;

  myChart.hideLoading();

  option = {
    title: {
      ...TITLE_DEFAULT,
      text: `Fee Distribution`,
    },
    color: [...CHART_COLORS],
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    grid: { ...GRID_DEFAULT },
    dataset: [
      {
        source: [fees],
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
      splitLine: {
        ...GRID_LINES,
        show: false,
      },
      splitArea: {
        show: true,
      },
      boundaryGap: true,
      nameGap: 30,
    },
    yAxis: [
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Fee Transfers (MINA)",
      },
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Outliers (MINA)",
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
        tooltip: {
          formatter: function (param) {
            return [
              ["max", param.data[5]],
              ["Q3", param.data[4]],
              ["median", param.data[3]],
              ["Q1", param.data[2]],
              ["min", param.data[1]],
            ]
              .map(([a, b]) => `<strong>${a}</strong>: ${b.toFixed(3)} MINA`)
              .join("</br>");
          },
        },
      },
      {
        name: "deviations",
        type: "scatter",
        symbolSize: 8,
        datasetIndex: 2,
        yAxisIndex: 1,
        tooltip: {
          formatter: function (param) {
            return [["Fee", param.data[1]]]
              .map(([a, b]) => `<strong>${a}</strong>: ${b.toFixed(3)} MINA`)
              .join("</br>");
          },
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

setTimeout(async () => {
  const groupSize = SLOT_GROUPING;
  let summary = await getBlockchainSummary();
  let blockheightLte = parseInt(
    getUrlParamOrDefault("q-blockheight-lte", summary.blockchainLength),
  );
  let blockheightGte = parseInt(
    getUrlParamOrDefault("q-blockheight-gte", summary.blockchainLength - 1000),
  );

  let boxAndWhiskerChart = echarts.init(document.getElementById("fee-spread"));
  let barPlot = echarts.init(document.getElementById("transfer-count"));
  [boxAndWhiskerChart, barPlot].forEach((chart) => {
    window.addEventListener("resize", function () {
      chart.resize();
    });
    chart.showLoading({
      text: "Loading...", // Display text with the spinner
      color: "#E39844", // Spinner color
      zlevel: 0,
    });
  });

  let response = await fetch(config.graphql_endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: `query InternalCommands(
          $limit: Int
          $sort_by: FeetransferSortByInput!
          $query: FeetransferQueryInput!
        ) {
          feetransfers(limit: $limit, sortBy: $sort_by, query: $query) {
            fee,
            blockHeight
          }
        },
      `,
      variables: {
        limit: 10000000,
        sort_by: "BLOCKHEIGHT_DESC",
        query: {
          canonical: true,
          blockHeight_gte: blockheightGte,
          blockHeight_lte: blockheightLte,
        },
      },
      operationName: "InternalCommands",
    }),
  });

  let jsonResp = await response.json();
  let data = jsonResp.data.feetransfers.reduce((agg, record) => {
    let key = record.blockHeight - (record.blockHeight % groupSize);
    key = `${key}-${key + groupSize - 1}`;
    if (!agg[key]) {
      agg[key] = [];
    }
    let fee = record.fee / 1e9;
    if (fee < 700) {
      agg[key].push(fee);
    }
    return agg;
  }, {});

  renderBoxAndWhiskerPlotIT(Object.values(data).flat(), boxAndWhiskerChart);
  renderTransferCountPlot(data, barPlot);
}, 1000);
