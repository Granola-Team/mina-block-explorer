setTimeout(async () => {
  const blockLimit = getBlockLimit();
  let { blockchainLength } = await getBlockchainSummary();
  const blockOffset = blockchainLength - blockLimit;

  let chartDom = document.getElementById("avg-snark-fee");
  window.addEventListener("resize", function () {
    myChart.resize();
  });
  let myChart = echarts.init(chartDom);

  myChart.showLoading({
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
      query: `query SnarkFees($limit: Int = 10, $sort_by: SnarkSortByInput!, $query: SnarkQueryInput!) {
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
          blockHeight_gte: blockOffset,
        },
      },
      operationName: "SnarkFees",
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
      data[height].totalFees = data[height].totalFees / 1e9; // nanomina to mina
      data[height].avgFee = data[height].totalFees / data[height].count;
    }
  }
  let counts = Object.values(data).map((e) => e.count);
  let avgFees = Object.values(data).map((e) => e.avgFee);

  let option;

  myChart.hideLoading();

  option = {
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    color: [...CHART_COLORS],
    title: {
      ...TITLE_DEFAULT,
      text: `Fees by block with averages`,
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
        name: "Avg Fee Per Block (MINA)",
        axisLabel: {
          ...Y_AXIS_AXIS_LABEL_DEFAULT,
          formatter: (value) => `${value}`,
        },
      },
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Fees Per Block",
        position: "right",
      },
    ],
    series: [
      {
        data: avgFees,
        type: "line",
        areaStyle: { ...SERIES_LINE_AREA_STYLES },
        yAxisIndex: 0,
        tooltip: {
          valueFormatter: (value) => `${value.toFixed(5)} MINA`,
        },
      },
      {
        data: counts,
        type: "scatter",
        yAxisIndex: 1,
        tooltip: {
          valueFormatter: (value) => `${value} fees`,
        },
      },
    ],
  };

  option && myChart.setOption(option);
}, 1000);
