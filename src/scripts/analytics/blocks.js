function renderTreeChart(data, myChart) {
  const MAX_DEPTH = 150;
  let option;

  myChart.hideLoading();

  let maxHeight = data[0].blockHeight;
  let fileredData = data.filter((b) => b.blockHeight > maxHeight - MAX_DEPTH);
  let tree = mapTreeToEchartsFormat(buildTree(fileredData));

  option = {
    tooltip: {
      position: "top",
    },
    title: {
      ...TITLE_DEFAULT,
      text: `Blockchain Tree`,
    },
    series: [
      {
        data: [tree],
        type: "tree",
        top: "1%",
        left: "50px",
        bottom: "1%",
        right: "50px",
        symbolSize: 20,
        symbol: "circle",
        layout: "orthogonal",
        orient: "LR",
        label: {
          position: "left",
          verticalAlign: "middle",
          align: "right",
          fontSize: 9,
        },
        leaves: {
          label: {
            position: "right",
            verticalAlign: "middle",
            align: "left",
          },
        },
        tooltip: {
          formatter: (params) => {
            return `<strong>${params.name}</strong>  |  ${params.data.canonical ? '<span style="color:#56D05F;">canonical</span>' : '<span style="color:#FB7631;">non-canonical</span>'}`;
          },
        },
        lineStyle: {
          width: 2,
          color: "#21252D",
        },
        itemStyle: {
          borderColor: "#21252D",
          borderWidth: 2,
        },
        emphasis: {
          focus: "relative",
          blurScope: "coordinateSystem",
        },
        expandAndCollapse: false,
        animationDuration: 0,
        animationDurationUpdate: 0,
      },
    ],
  };

  option && myChart.setOption(option);
  myChart.on("click", function (params) {
    if (params.data.stateHash) {
      window.open("/blocks/" + params.data.stateHash, "_blank");
    }
  });
}

function renderCanonicalVsNonCanonicalChart(data, myChart) {
  let option;

  myChart.hideLoading();

  const slots = data.map(([slot]) => slot);
  const canonical_blocks = data.map(([_slot, canonical]) => canonical);
  const non_canonical_blocks = data.map(
    ([_slot, _canonical, non_canonical]) => non_canonical,
  );

  option = {
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    color: [...CHART_COLORS],
    title: {
      ...TITLE_DEFAULT,
      text: "Blocks",
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      ...X_AXIS_DEFAULT,
      type: "category",
      name: "Block Height",
      data: slots,
      splitLine: {
        ...GRID_LINES,
        show: false,
      },
      axisLabel: {
        ...X_AXIS_LABEL_DEFAULT,
        formatter: (v) => parseInt(v.split("-")[0]),
      },
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

  const slots = data.map(([slot]) => slot);
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
      name: "Block Height",
      data: slots,
      axisLabel: {
        ...X_AXIS_LABEL_DEFAULT,
        formatter: (v) => parseInt(v.split("-")[0]),
      },
    },
    yAxis: {
      type: "value",
      name: "Coinbase Rewards",
      ...Y_AXIS_DEFAULT,
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        formatter: nanominaToKMina,
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
          valueFormatter: (value) => nanominaToKMina(value) + " MINA",
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
  const groupSize = SLOT_GROUPING;

  let rewardsChart = echarts.init(document.getElementById("rewards"));
  let blocksChart = echarts.init(document.getElementById("blocks"));
  let treeChart = echarts.init(document.getElementById("tree"));

  [rewardsChart, blocksChart, treeChart].forEach((chart) => {
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
          stateHash
          protocolState {
            previousStateHash
          }
        }
      }`,
      variables: {
        limit: 1e9,
        sort_by: "BLOCKHEIGHT_DESC",
        query: {
          blockHeight_gte: blockheightGte,
          blockHeight_lte: blockheightLte,
        },
      },
      operationName: "BlocksQuery",
    }),
  });

  let jsonResp = await response.json();
  let unique_creators = {};
  let data = jsonResp.data.blocks.reduce((agg, record) => {
    let key = record.blockHeight - (record.blockHeight % groupSize);
    key = `${key}-${key + groupSize - 1}`;
    let value = record.transactions.coinbase;
    if (!unique_creators[record.creator]) {
      unique_creators[record.creator] = 0;
    }
    if (!agg[key]) {
      agg[key] = {
        reward_sum: 0,
        canonical_blocks_count: 0,
        non_canonical_blocks_count: 0,
      };
    }

    if (record.canonical) {
      unique_creators[record.creator] += 1;
      agg[key].canonical_blocks_count += 1;
      agg[key].reward_sum += +value;
    } else {
      agg[key].non_canonical_blocks_count += 1;
    }
    return agg;
  }, {});

  let rewards_data = Object.entries(data).map(([key, val]) => [
    key,
    val.reward_sum,
  ]);
  rewards_data.sort(
    (a, b) => parseInt(a[0].split("-")[0]) - parseInt(b[0].split("-")[0]),
  );

  let blocks_data = Object.entries(data).map(([key, val]) => [
    key,
    val.canonical_blocks_count,
    val.non_canonical_blocks_count,
  ]);
  blocks_data.sort(
    (a, b) => parseInt(a[0].split("-")[0]) - parseInt(b[0].split("-")[0]),
  );

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

  renderTreeChart(jsonResp.data.blocks, treeChart);
  renderCoinbaseRewardsChart(rewards_data, rewardsChart);
  renderCanonicalVsNonCanonicalChart(blocks_data, blocksChart);
}, 1000);
