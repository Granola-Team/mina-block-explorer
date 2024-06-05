setTimeout(async () => {
  const blockLimit = 3000;
  const groupSize = 10;
  let response = await fetch(config.graphql_endpoint, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      query: `query TransactionsQuery(
              $limit: Int
              $sort_by: TransactionSortByInput!
              $query: TransactionQueryInput!
          ) {
              transactions(limit: $limit, sortBy: $sort_by, query: $query) {
              hash
              amount
              kind
              blockHeight
              block {
                  dateTime
              }
              }
          }`,
      variables: {
        limit: blockLimit,
        sort_by: "BLOCKHEIGHT_DESC",
        query: {
          canonical: true,
          kind: "PAYMENT",
        },
      },
      operationName: "TransactionsQuery",
    }),
  });

  let jsonResp = await response.json();
  // Use reduce to aggregate transaction count and total amount per day
  const data = jsonResp.data.transactions.reduce((acc, transaction) => {
    // Convert dateTime to a Date object and adjust it to the start of the day
    const date = new Date(transaction.block.dateTime);
    date.setUTCHours(0, 0, 0, 0);

    // Convert adjusted date to a Unix timestamp (milliseconds since Unix Epoch divided by 1000 for seconds)
    const unixTimestamp = Math.floor(date.getTime() / 1000);

    // Initialize the date key if not already present
    if (!acc[unixTimestamp]) {
      acc[unixTimestamp] = { count: 0, totalAmount: 0 };
    }

    // Increment count and add to total amount
    acc[unixTimestamp].count += 1;
    acc[unixTimestamp].totalAmount += transaction.amount;

    return acc;
  }, {});

  let dates = Object.keys(data).map((key) =>
    new Date(parseInt(key) * 1000).toISOString().substring(0, 10),
  ); // Convert back to milliseconds for chart
  let txnVolume = Object.values(data).map((value) => value.count);

  console.log(data, dates, txnVolume);

  let chartDom = document.getElementById("chart");
  window.addEventListener("resize", function () {
    myChart.resize();
  });
  let myChart = echarts.init(chartDom);
  let option;

  option = {
    title: {
      text: `Transaction Volume of Last ${blockLimit} Blocks by Day`,
      left: "center",
    },
    xAxis: {
      type: "category",
      data: dates,
    },
    yAxis: {
      type: "value",
      name: "Txn Count",
    },
    series: [
      {
        data: txnVolume,
        type: "line",
        areaStyle: {},
      },
    ],
  };

  console.log(option);

  option && myChart.setOption(option);
}, 1000);
