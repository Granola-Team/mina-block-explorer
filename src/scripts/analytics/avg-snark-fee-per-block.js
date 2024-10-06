function renderFeeDistributionChart(data, myChart) {
  let option;

  myChart.hideLoading();

  let fees = Object.keys(data).map((f) => +f);
  let counts = Object.values(data);

  option = {
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    color: [...CHART_COLORS],
    title: {
      ...TITLE_DEFAULT,
      text: `Fee Distribution (Non-Zero Fees)`,
    },
    grid: { ...GRID_DEFAULT },
    yAxis: {
      ...Y_AXIS_DEFAULT,
      type: "category",
      data: fees,
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        formatter: (value) => scaleMina(value),
      },
    },
    xAxis: [
      {
        ...X_AXIS_DEFAULT,
        type: "value",
        name: "Instances of Fee Amount",
        min: 0,
      },
    ],
    series: [
      {
        name: "Instances of fee amount",
        data: counts,
        type: "scatter",
        tooltip: {
          formatter: function (params) {
            return `${params.value} instances of ${scaleMina(params.name)}`;
          },
        },
        label: {
          show: true,
          position: "right",
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
  let blockheightLte = parseInt(getUrlParam("q-blockheight-lte"));
  let blockheightGte = parseInt(getUrlParam("q-blockheight-gte"));

  let avgFeeChart = echarts.init(document.getElementById("avg-snark-fee"));
  let feePerBlockChart = echarts.init(
    document.getElementById("fees-per-block"),
  );
  let feeDistributionChart = echarts.init(
    document.getElementById("fee-distribution"),
  );
  [avgFeeChart, feePerBlockChart, feeDistributionChart].forEach((chart) => {
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
  renderFeeDistributionChart(feeDist, feeDistributionChart);
  renderAveFeePerBlock(avgFees, heights, avgFeeChart);
  renderTotalFeesPerBlock(totalFees, heights, feePerBlockChart);
}, 1000);
