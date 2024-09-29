function renderTopTransfersChart(data, myChart) {
  let option;

  myChart.hideLoading();

  option = {
    tooltip: {
      ...TOOLTIP_DEFAULT,
    },
    title: {
      text: `Transfers with highest values`,
      ...TITLE_DEFAULT,
    },
    grid: { ...GRID_DEFAULT },
    xAxis: {
      type: "value",
      name: "Txn amount (MINA)",
      ...X_AXIS_DEFAULT,
      axisLabel: {
        ...X_AXIS_LABEL_DEFAULT,
        formatter: (value) => (value / 1e12).toFixed(2) + "k",
      },
    },
    yAxis: {
      ...Y_AXIS_DEFAULT,
      type: "category",
      data: data.map(([hash, _amount]) => hash),
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        formatter: (value) => "..." + value.slice(-6),
      },
    },
    series: [
      {
        data: data.map(([_hash, amount]) => amount),
        type: "bar",
      },
    ],
  };

  const HASH_LEN = 53;
  myChart.on("click", function (params) {
    if (params.name && params.name.length == HASH_LEN) {
      window.open("/commands/" + params.name, "_blank");
    }
  });

  option && myChart.setOption(option);
}

function renderTopRecipientsChart(dataMap, myChart) {
  let data = Object.entries(dataMap);
  data.sort((a, b) => b[1] - a[1]); // descending

  data = data.slice(0, 10);
  data.reverse();

  let option;

  myChart.hideLoading();

  option = {
    tooltip: { ...TOOLTIP_DEFAULT },
    title: {
      ...TITLE_DEFAULT,
      text: `Most frequent recipients`,
    },
    xAxis: {
      type: "value",
      name: "Txn recieved",
      ...X_AXIS_DEFAULT,
    },
    yAxis: {
      ...Y_AXIS_DEFAULT,
      type: "category",
      data: data.map(([recipient, _count]) => recipient),
      axisLabel: {
        ...Y_AXIS_AXIS_LABEL_DEFAULT,
        formatter: (value) => "..." + value.slice(-6),
      },
    },
    series: [
      {
        ...BAR_SERIES_DEFAULT,
        data: data.map(([_recipient, count]) => count),
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

function renderTransactionVolumeChart(data, myChart) {
  let dates = Object.keys(data).map((key) =>
    new Date(parseInt(key) * 1000).toISOString().substring(0, 10),
  ); // Convert back to milliseconds for chart
  let txnVolume = Object.values(data).map((value) => value.count);
  let amounts = Object.values(data).map((value) => value.totalAmount);

  let option;

  myChart.hideLoading();

  option = {
    tooltip: { ...TOOLTIP_DEFAULT },
    title: {
      ...TITLE_DEFAULT,
      text: `Transaction volume by day`,
    },
    xAxis: {
      ...X_AXIS_DEFAULT,
      type: "category",
      data: dates,
    },
    yAxis: [
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Txn Count",
      },
      {
        ...Y_AXIS_DEFAULT,
        type: "value",
        name: "Txn Amount (millions of MINA)",
        position: "right",
        axisLabel: {
          ...Y_AXIS_AXIS_LABEL_DEFAULT,
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
        ...BAR_SERIES_DEFAULT,
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
  const blockLimit = getBlockLimit();
  let summary_response = await fetch(config.rest_endpoint + "/summary", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  let { blockchainLength } = await summary_response.json();
  const blockOffset = blockchainLength - blockLimit;
  let volumeChartDom = document.getElementById("user-commands-volume");
  let topRecipientsChartDom = document.getElementById(
    "user-commands-top-recipients",
  );
  let topTransfersChartDom = document.getElementById(
    "user-commands-top-transfers",
  );
  window.addEventListener("resize", function () {
    volumeChart.resize();
  });
  let volumeChart = echarts.init(volumeChartDom);
  let topRecipientsChart = echarts.init(topRecipientsChartDom);
  let topTransfersChart = echarts.init(topTransfersChartDom);

  volumeChart.showLoading({
    text: "Loading...", // Display text with the spinner
    color: "#E39844", // Spinner color
    zlevel: 0,
  });
  topRecipientsChart.showLoading({
    text: "Loading...", // Display text with the spinner
    color: "#E39844", // Spinner color
    zlevel: 0,
  });
  topTransfersChart.showLoading({
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
    recipients_count: {},
    largest_transfers: [],
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

    // keep top 10 largest transfers
    stats.largest_transfers.push([transaction.hash, transaction.amount]);
    stats.largest_transfers.sort((a, b) => b[1] - a[1]); //descending
    stats.largest_transfers = stats.largest_transfers.slice(
      0,
      Math.min(stats.largest_transfers.length, 10),
    );

    stats.total_transferred += transaction.amount;
    stats.total_number_of_transactions += 1;
    stats.total_fees += transaction.fee;
    if (transaction.failureReason === "Amount_insufficient_to_create_account") {
      stats.total_failed_account_creations += 1;
    }

    if (!stats.recipients_count[transaction.receiver.publicKey]) {
      stats.recipients_count[transaction.receiver.publicKey] = 0;
    }

    stats.recipients_count[transaction.receiver.publicKey] += 1;

    return acc;
  }, {});

  stats.largest_transfers.reverse();

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
  renderTopRecipientsChart(stats.recipients_count, topRecipientsChart);
  renderTopTransfersChart(stats.largest_transfers, topTransfersChart);
}, 1000);
