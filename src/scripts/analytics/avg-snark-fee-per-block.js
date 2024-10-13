function renderTopSNARKWorkersChart(dataMap, myChart) {
  let data = Object.entries(dataMap);
  data.sort((a, b) => b[1].count - a[1].count); // descending

  data = data.slice(0, 10);
  data.reverse();

  let option;

  myChart.hideLoading();

  option = {
    tooltip: { ...TOOLTIP_DEFAULT },
    title: {
      ...TITLE_DEFAULT,
      text: `Top SNARK Provers`,
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      type: "value",
      name: "SNARKs Proved",
      ...X_AXIS_DEFAULT,
    },
    yAxis: {
      ...Y_AXIS_DEFAULT,
      type: "category",
      data: data.map(([prover, _count]) => prover),
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        showMinLabel: true,
        formatter: (_, zero_based_index) => getOrdinal(10 - zero_based_index),
      },
      splitLine: {
        ...GRID_LINES,
        show: false,
      },
    },
    series: [
      {
        ...BAR_SERIES_DEFAULT,
        data: data.map(([_prover, data]) => data.count),
        type: "bar",
      },
    ],
  };

  const PUBLIC_KEY_LEN = 55;
  myChart.on("click", function (params) {
    if (params.name && params.name.length == PUBLIC_KEY_LEN) {
      window.open("/addresses/accounts/" + params.name, "_blank");
    }
  });

  option && myChart.setOption(option);
}

function renderTopSNARKEarnersChart(dataMap, myChart) {
  let data = Object.entries(dataMap);
  data.sort((a, b) => b[1].totalFees - a[1].totalFees); // descending

  data = data.slice(0, 10);
  data.reverse();
  data = data.map(([prover, data]) => [
    prover,
    {
      ...data,
      totalFees: data.totalFees / 1e9, // nanomina to mina conversion
    },
  ]);

  let option;

  myChart.hideLoading();

  option = {
    tooltip: { ...TOOLTIP_DEFAULT },
    title: {
      ...TITLE_DEFAULT,
      text: `Top SNARK Earners`,
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      type: "value",
      name: "Earned (Mina)",
      ...X_AXIS_DEFAULT,
    },
    yAxis: {
      ...Y_AXIS_DEFAULT,
      type: "category",
      data: data.map(([prover, _data]) => prover),
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        showMinLabel: true,
        formatter: (_, zero_based_index) => getOrdinal(10 - zero_based_index),
      },
      splitLine: {
        ...GRID_LINES,
        show: false,
      },
    },
    series: [
      {
        ...BAR_SERIES_DEFAULT,
        data: data.map(([_prover, data]) => data.totalFees),
        type: "bar",
        tooltip: {
          valueFormatter: (v) => scaleMina(v * 1e9), //scale mina down to nanomina,
        },
      },
    ],
  };

  const PUBLIC_KEY_LEN = 55;
  myChart.on("click", function (params) {
    if (params.name && params.name.length == PUBLIC_KEY_LEN) {
      window.open("/addresses/accounts/" + params.name, "_blank");
    }
  });

  option && myChart.setOption(option);
}

function renderSnarkJobsChart(data, myChart) {
  let dates = Object.keys(data)
    .map(unixTimestampToDateString)
    .map(dayAxisLabelFormatter);
  let snarkJobs = Object.values(data);

  let option;

  myChart.hideLoading();

  option = {
    tooltip: { ...TOOLTIP_DEFAULT },
    color: [...CHART_COLORS],
    title: {
      ...TITLE_DEFAULT,
      text: `SNARK volume by day`,
    },

    grid: { ...GRID_DEFAULT },
    xAxis: {
      ...X_AXIS_DEFAULT,
      type: "category",
      data: dates,
      splitLine: {
        ...GRID_LINES,
        show: false,
      },
    },
    yAxis: [
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "SNARK Job Count",
      },
    ],
    series: [
      {
        data: snarkJobs,
        type: "bar",
        tooltip: {
          valueFormatter: (v) => `${v} jobs`,
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

function renderBoxAndWhiskerPlot(fees, myChart) {
  let option;

  fees = fees.map((f) => f / 1e9); // convert to mina

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
        name: "Fee (MINA)",
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

function renderTotalFeesPerBlock(data, heights, myChart) {
  let option;

  myChart.hideLoading();

  option = {
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    color: [...CHART_COLORS],
    title: {
      ...TITLE_DEFAULT,
      text: `Total Fees Per Block `,
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      ...X_AXIS_DEFAULT,
      type: "category",
      name: "Block Height",
      data: heights,
    },
    yAxis: [
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Fee Amount",
        axisLabel: {
          ...Y_AXIS_AXIS_LABEL_DEFAULT,
          formatter: (value) => `${(value / 1e9).toFixed(0)}`,
        },
      },
    ],
    series: [
      {
        data: data,
        type: "bar",
        tooltip: {
          valueFormatter: (value) => scaleMina(value),
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

function renderAveFeePerBlock(data, heights, myChart) {
  let option;

  myChart.hideLoading();

  option = {
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    color: [...CHART_COLORS],
    title: {
      ...TITLE_DEFAULT,
      text: `Average Fee Per Block`,
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      ...X_AXIS_DEFAULT,
      type: "category",
      name: "Block Height",
      data: heights,
    },
    yAxis: [
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Fee (MINA)",
        axisLabel: {
          ...Y_AXIS_AXIS_LABEL_DEFAULT,
          formatter: (value) => `${(value / 1e9).toFixed(0)}`,
        },
      },
    ],
    series: [
      {
        data: data,
        type: "bar",
        tooltip: {
          valueFormatter: (value) => scaleMina(value),
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

setTimeout(async () => {
  let summary = await getBlockchainSummary();

  let blockheightLte = parseInt(
    getUrlParamOrDefault("q-blockheight-lte", summary.blockchainLength),
  );
  let blockheightGte = parseInt(
    getUrlParamOrDefault("q-blockheight-gte", summary.blockchainLength - 1000),
  );

  let avgFeeChart = echarts.init(document.getElementById("avg-snark-fee"));
  let feePerBlockChart = echarts.init(
    document.getElementById("fees-per-block"),
  );
  let feeDistributionChart = echarts.init(
    document.getElementById("fee-distribution"),
  );
  let snarkJobsChart = echarts.init(
    document.getElementById("snark-jobs-count"),
  );
  let topSnarkProversChart = echarts.init(
    document.getElementById("top-snark-provers"),
  );
  let topSNARKWorkersChart = echarts.init(
    document.getElementById("top-snark-workers"),
  );
  [
    avgFeeChart,
    feePerBlockChart,
    feeDistributionChart,
    snarkJobsChart,
    topSnarkProversChart,
    topSNARKWorkersChart,
  ].forEach((chart) => {
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
      query: `query SnarkFeesAnalyticsQuery($limit: Int = 10, $sort_by: SnarkSortByInput!, $query: SnarkQueryInput!) {
        snarks(limit: $limit, sortBy: $sort_by, query: $query ) {
          fee
          blockHeight
          dateTime
          prover
        }
      }`,
      variables: {
        limit: 1e9, // very large limit to make block height the effective limit
        sort_by: "BLOCKHEIGHT_DESC",
        query: {
          canonical: true,
          blockHeight_gte: blockheightGte,
          blockHeight_lte: blockheightLte,
        },
      },
      operationName: "SnarkFeesAnalyticsQuery",
    }),
  });

  let jsonResp = await response.json();
  // Use reduce to aggregate transaction count and total amount per day
  const data = jsonResp.data.snarks.reduce((acc, snark) => {
    let key = snark.blockHeight;
    if (!acc[key]) {
      acc[key] = { count: 0, totalFees: 0 };
    }

    // Increment count and add to total amount
    acc[key].count += 1;
    acc[key].totalFees += snark.fee;

    return acc;
  }, {});
  const topSnarkEarners = jsonResp.data.snarks.reduce((acc, snark) => {
    let key = snark.prover;
    if (!acc[key]) {
      acc[key] = { count: 0, totalFees: 0 };
    }

    // Increment count and add to total amount
    acc[key].count += 1;
    acc[key].totalFees += snark.fee;

    return acc;
  }, {});
  const countsByDay = jsonResp.data.snarks.reduce((acc, snark) => {
    let key = getUnixTimestampTruncatedToDay(snark.dateTime);
    if (!acc[key]) {
      acc[key] = 0;
    }

    acc[key] += 1;

    return acc;
  }, {});

  const feeDist = jsonResp.data.snarks.reduce((acc, snark) => {
    let key = snark.fee;
    if (!acc[key]) {
      acc[key] = 0;
    }

    acc[snark.fee] += 1;
    return acc;
  }, {});

  let heights = Object.keys(data).sort();
  for (const height of heights) {
    if (data[height].count != 0) {
      data[height].totalFees = data[height].totalFees;
      data[height].avgFee = data[height].totalFees / data[height].count;
    }
  }
  let totalFees = Object.values(data).map((e) => e.totalFees);
  let avgFees = Object.values(data).map((e) => e.avgFee);

  const fees = Object.keys(feeDist)
    .map((f) => +f)
    .sort();
  const [fee, unit] = scaleMina(Math.max(...fees)).split(" ");

  document.getElementById("fee-free-work").innerHTML =
    new Intl.NumberFormat().format(feeDist["0"]);
  document.getElementById("total-snark-jobs").innerHTML =
    new Intl.NumberFormat().format(jsonResp.data.snarks.length);
  document.getElementById("for-fee-jobs").innerHTML =
    new Intl.NumberFormat().format(jsonResp.data.snarks.length - +feeDist["0"]);
  document.getElementById("highest-fee").innerHTML =
    new Intl.NumberFormat().format(fee, { style: "currency" });
  document
    .getElementById("highest-fee")
    .parentElement.querySelector(".subtext").innerHTML = `in ${unit}`;

  delete feeDist["0"];

  renderTopSNARKEarnersChart(topSnarkEarners, topSnarkProversChart);
  renderTopSNARKWorkersChart(topSnarkEarners, topSNARKWorkersChart);
  renderSnarkJobsChart(countsByDay, snarkJobsChart);
  renderBoxAndWhiskerPlot(fees, feeDistributionChart);
  renderAveFeePerBlock(avgFees, heights, avgFeeChart);
  renderTotalFeesPerBlock(totalFees, heights, feePerBlockChart);
}, 1000);
