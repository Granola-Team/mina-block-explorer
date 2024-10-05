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
          formatter: (value) => `${(value / 1e9).toFixed(3)}`,
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
      text: `Ave Fee Per Block`,
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
          formatter: (value) => `${(value / 1e9).toFixed(3)}`,
        },
      },
    ],
    series: [
      {
        data: data,
        type: "bar",
        tooltip: {
          valueFormatter: (value) => scaleMina(mina),
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

setTimeout(async () => {
  let blockheightLte = parseInt(getUrlParam("q-blockheight-lte"));
  let blockheightGte = parseInt(getUrlParam("q-blockheight-gte"));
  let avgFeeDom = document.getElementById("avg-snark-fee");
  let feePerBlockDom = document.getElementById("fees-per-block");
  window.addEventListener("resize", function () {
    avgFeeChart.resize();
    feePerBlockChart.resize();
  });
  let avgFeeChart = echarts.init(avgFeeDom);
  let feePerBlockChart = echarts.init(feePerBlockDom);

  avgFeeChart.showLoading({
    text: "Loading...", // Display text with the spinner
    color: "#E39844", // Spinner color
    zlevel: 0,
  });

  feePerBlockChart.showLoading({
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

  let heights = Object.keys(data).sort();
  for (const height of heights) {
    if (data[height].count != 0) {
      data[height].totalFees = data[height].totalFees;
      data[height].avgFee = data[height].totalFees / data[height].count;
    }
  }
  let totalFees = Object.values(data).map((e) => e.totalFees);
  let avgFees = Object.values(data).map((e) => e.avgFee);

  renderAveFeePerBlock(avgFees, heights, avgFeeChart);
  renderTotalFeesPerBlock(totalFees, heights, feePerBlockChart);
}, 1000);
