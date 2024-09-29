function renderCanonicalVsNonCanonicalChart(data, myChart) {
  let option;

  myChart.hideLoading();

  const slots = data.map(([slot]) => parseInt(slot));
  const canonical_blocks = data.map(([_slot, canonical]) => canonical);
  const non_canonical_blocks = data.map(
    ([_slot, _canonical, non_canonical]) => non_canonical,
  );

  option = {
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    title: {
      ...TITLE_DEFAULT,
      text: "Blocks",
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      ...X_AXIS_DEFAULT,
      type: "category",
      name: "Global Slot",
      data: slots,
    },
    yAxis: {
      type: "value",
      name: "Block Count",
      ...Y_AXIS_DEFAULT,
    },
    series: [
      {
        data: non_canonical_blocks,
        type: "bar",
        stack: "block",
        name: "Non-canonical Blocks",
      },
      {
        ...BAR_SERIES_DEFAULT,
        data: canonical_blocks,
        type: "bar",
        stack: "block",
        name: "Canonical Blocks",
      },
    ],
  };

  option && myChart.setOption(option);
}

function renderCoinbaseRewardsChart(data, myChart) {
  let option;

  myChart.hideLoading();

  const slots = data.map(([slot]) => parseInt(slot));
  const rewards = data.map(([_slot, reward]) => reward);

  option = {
    tooltip: {
      position: "top",
    },
    title: {
      ...TITLE_DEFAULT,
      text: `Coinbase Rewards`,
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      ...X_AXIS_DEFAULT,
      type: "category",
      name: "Global Slot",
      data: slots,
    },
    yAxis: {
      type: "value",
      name: "Coinbase Rewards",
      ...Y_AXIS_DEFAULT,
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        formatter: (value) => `${(value / 1e12).toFixed(0)}k`,
      },
    },
    series: [
      {
        data: rewards,
        type: "line",
        areaStyle: { ...SERIES_LINE_AREA_STYLES },
        smooth: true,
        yAxisIndex: 0,
        tooltip: {
          valueFormatter: (value) => `${(value / 1e12).toFixed(0)}k MINA`,
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

setTimeout(async () => {
  const blockLimit = getBlockLimit();
  let summary_response = await fetch(config.rest_endpoint + "/summary", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  let { blockchainLength } = await summary_response.json();
  const blockOffset = blockchainLength - blockLimit;
  const groupSize = 120;

  let rewardsChartDom = document.getElementById("rewards");
  let blocksChartDom = document.getElementById("blocks");
  window.addEventListener("resize", function () {
    rewardsChart.resize();
    blocksChart.resize();
  });
  let rewardsChart = echarts.init(rewardsChartDom);
  let blocksChart = echarts.init(blocksChartDom);

  rewardsChart.showLoading({
    text: "Loading...", // Display text with the spinner
    color: "#E39844", // Spinner color
    zlevel: 0,
  });

  blocksChart.showLoading({
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
      query: `query BlocksQuery(
            $query: BlockQueryInput!
            $limit: Int = 10
            $sort_by: BlockSortByInput!
      ) {
        blocks(query: $query, limit: $limit, sortBy: $sort_by) {
          blockHeight
          globalSlotSinceGenesis
          transactions {
            coinbase
          }
          creator
          snarkFees
          txFees
          canonical
        }
      }`,
      variables: {
        limit: 1e9,
        sort_by: "BLOCKHEIGHT_DESC",
        query: {
          blockHeight_gte: blockOffset,
        },
      },
      operationName: "BlocksQuery",
    }),
  });

  let jsonResp = await response.json();
  let unique_creators = {};
  let data = jsonResp.data.blocks.reduce((agg, record) => {
    if (!record.canonical) return agg;
    let slot = record.globalSlotSinceGenesis;
    let key = slot - (slot % groupSize);
    let value = record.transactions.coinbase;
    if (!unique_creators[record.creator]) {
      unique_creators[record.creator] = 0;
    }
    unique_creators[record.creator] += 1;
    if (!agg[key]) {
      agg[key] = {
        reward_sum: 0,
        canonical_blocks_count: 0,
        non_canonical_blocks_count: 0,
      };
    }
    agg[key].reward_sum += +value;
    if (record.canonical == true) {
      agg[key].canonical_blocks_count += 1;
    } else {
      agg[key].non_canonical_blocks_count += 1;
    }
    return agg;
  }, {});

  // trim the first slot group and last slot group
  // as they most likely will not contain a full data set
  let keys = Object.keys(data).map((k) => parseInt(k));
  keys.sort((a, b) => a - b); // sort the values asc
  delete data[keys[0]];
  delete data[keys[keys.length - 1]];

  let rewards_data = Object.entries(data).map(([key, val]) => [
    key,
    val.reward_sum,
  ]);

  let blocks_data = Object.entries(data).map(([key, val]) => [
    key,
    val.canonical_blocks_count,
    val.non_canonical_blocks_count,
  ]);

  document.getElementById("canonical-blocks-count").innerHTML = Object.values(
    data,
  ).reduce((agg, { canonical_blocks_count }) => {
    agg += canonical_blocks_count;
    return agg;
  }, 0);
  document.getElementById("non-canonical-blocks-count").innerHTML =
    Object.values(data).reduce((agg, { non_canonical_blocks_count }) => {
      agg += non_canonical_blocks_count;
      return agg;
    }, 0);
  document.getElementById("unique-block-producers-count").innerHTML =
    Object.keys(unique_creators).length;

  renderCoinbaseRewardsChart(rewards_data, rewardsChart);
  renderCanonicalVsNonCanonicalChart(blocks_data, blocksChart);
}, 1000);
