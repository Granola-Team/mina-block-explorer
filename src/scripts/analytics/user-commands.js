function renderTransactionVolumeChart(data, myChart) {
  let dates = Object.keys(data).map((key) =>
    new Date(parseInt(key) * 1000).toISOString().substring(0, 10),
  ); // Convert back to milliseconds for chart
  let txnVolume = Object.values(data).map((value) => value.count);
  let amounts = Object.values(data).map((value) => value.totalAmount);

  let option;

  myChart.hideLoading();

  option = {
    tooltip: {
      position: "top",
    },
    title: {
      text: `Transaction volume by day`,
      left: "center",
    },
    xAxis: {
      type: "category",
      data: dates,
    },
    yAxis: [
      {
        type: "value",
        name: "Txn Count",
      },
      {
        type: "value",
        name: "Txn Amount (millions of MINA)",
        position: "right",
        axisLabel: {
          formatter: (value) => `${(value / 1e15).toFixed(2)}`,
        },
      },
    ],
    series: [
      {
        data: txnVolume,
        type: "line",
        yAxisIndex: 0,
        smooth: true,
        tooltip: {
          valueFormatter: (value) => `${value} txn`,
        },
      },
      {
        data: amounts,
        type: "bar",
        yAxisIndex: 1,
        barMaxWidth: 20,
        tooltip: {
          valueFormatter: (value) =>
            `${(value / 1e15).toFixed(2)} million MINA`,
        },
      },
    ],
  };

  option && myChart.setOption(option);
}

setTimeout(async () => {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const blockLimit = parseInt(urlParams.get("limit")) || 1000;
  let summary_response = await fetch(config.rest_endpoint + "/summary", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  let { blockchainLength } = await summary_response.json();
  const blockOffset = blockchainLength - blockLimit;
  let chartDom = document.getElementById("user-commands-volume");
  window.addEventListener("resize", function () {
    volumeChart.resize();
  });
  let volumeChart = echarts.init(chartDom);

  volumeChart.showLoading({
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
      query: `query TransactionsQuery(
              $limit: Int
              $sort_by: TransactionSortByInput!
              $query: TransactionQueryInput!
      ) {
        transactions(limit: $limit, sortBy: $sort_by, query: $query) {
          hash
          amount
          kind
          fee
          receiver {
            publicKey
          }
          blockHeight
          failureReason
          block {
            dateTime
          }
        }
      }`,
      variables: {
        limit: 1e9,
        sort_by: "BLOCKHEIGHT_DESC",
        query: {
          canonical: true,
          kind: "PAYMENT",
          blockHeight_gte: blockOffset,
        },
      },
      operationName: "TransactionsQuery",
    }),
  });

  let stats = {
    total_transferred: 0,
    total_fees: 0,
    total_number_of_transactions: 0,
    total_failed_account_creations: 0,
    reciepients_count: {},
  };

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

    stats.total_transferred += transaction.amount;
    stats.total_number_of_transactions += 1;
    stats.total_fees += transaction.fee;
    if (transaction.failureReason === "Amount_insufficient_to_create_account") {
      stats.total_failed_account_creations += 1;
    }

    if (!stats.reciepients_count[transaction.receiver.publicKey]) {
      stats.reciepients_count[transaction.receiver.publicKey] = 0;
    }

    stats.reciepients_count[transaction.receiver.publicKey] += 1;

    return acc;
  }, {});

  document.getElementById("total-transferred").innerHTML =
    new Intl.NumberFormat().format(stats.total_transferred / 1e15, {
      style: "currency",
    }) + " million MINA";
  document.getElementById("total-fees").innerHTML =
    new Intl.NumberFormat().format(stats.total_fees / 1e9, {
      style: "currency",
    }) + " MINA";
  document.getElementById("total-number-of-transactions").innerHTML =
    new Intl.NumberFormat().format(stats.total_number_of_transactions);
  document.getElementById("total-failed-account-creations").innerHTML =
    new Intl.NumberFormat().format(stats.total_failed_account_creations);

  renderTransactionVolumeChart(data, volumeChart);
}, 1000);
