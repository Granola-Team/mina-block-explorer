setTimeout(async () => {
  const blockLimit = 2000;
  const groupSize = 50;

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
              snarkFees
              txFees
            }
          }`,
      variables: {
        limit: blockLimit,
        sort_by: "BLOCKHEIGHT_DESC",
        query: {
          canonical: true,
        },
      },
      operationName: "BlocksQuery",
    }),
  });

  let jsonResp = await response.json();
  let data = jsonResp.data.blocks.reduce((agg, record) => {
    let slot = record.globalSlotSinceGenesis;
    let key = slot - (slot % groupSize);
    let value = record.transactions.coinbase;
    if (!agg[key]) {
      agg[key] = 0;
    }
    agg[key] += +value;
    return agg;
  }, {});

  let option;

  myChart.hideLoading();

  option = {
    title: {
      text: `Coinbase Rewards in the last ${blockLimit} blocks`,
      left: "center",
    },
    xAxis: {
      type: "category",
      name: "Global Slot",
      data: Object.keys(data),
    },
    yAxis: {
      type: "value",
      name: "Coinbase Rewards",
      axisLabel: {
        formatter: (value) => `${(value / 1e12).toFixed(2)}k MINA`, // Display values in trillions
      },
    },

    series: [
      {
        data: Object.values(data),
        type: "line",
        areaStyle: {},
        yAxisIndex: 0,
      },
    ],
  };

  console.log(option);

  option && myChart.setOption(option);
}, 1000);
