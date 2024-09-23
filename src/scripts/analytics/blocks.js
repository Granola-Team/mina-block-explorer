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
      text: `Coinbase Rewards`,
      left: "center",
    },
    xAxis: {
      type: "category",
      name: "Global Slot",
      data: slots,
    },
    yAxis: {
      type: "value",
      name: "Coinbase Rewards",
      axisLabel: {
        formatter: (value) => `${(value / 1e12).toFixed(0)}k MINA`, // Display values in trillions
      },
    },
    series: [
      {
        data: rewards,
        type: "line",
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
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const blockLimit = urlParams.get("limit") || 1000;
  let summary_response = await fetch(config.rest_endpoint + "/summary", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  let { blockchainLength } = await summary_response.json();
  const blockOffset = blockchainLength - blockLimit;
  const groupSize = 120;

  let chartDom = document.getElementById("chart");
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
    record.canonical
      ? (agg[key].canonical_blocks_count += 1)
      : (canonical_blocks_count.non_canonical_blocks_count += 1);
    return agg;
  }, {});

  // trim the first slot and last slot
  // as they most likely will not contain
  // a full data set
  let keys = Object.keys(data).map((k) => parseInt(k));
  keys.sort((a, b) => a - b); // sort the values asc
  delete data[keys[0]];
  delete data[keys[keys.length - 1]];

  let rewards_data = Object.entries(data).map(([key, val]) => [
    key,
    val.reward_sum,
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

  renderCoinbaseRewardsChart(rewards_data, myChart);
}, 1000);
