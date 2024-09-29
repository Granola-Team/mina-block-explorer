const TOOLTIP_DEFAULT = { position: "top" };
const AXIS_NAME_TEXT_STYLE = {
  fontWeight: 700,
  fontFamily: "system-ui",
  fontSize: 12,
  lineHeight: 16,
  color: "#21252D",
};
const TITLE_DEFAULT = {
  left: 15,
  textStyle: {
    fontWeight: 600,
    fontFamily: "system-ui",
    fontSize: 17,
    lineHeight: 20.57,
    color: "#21252D",
  },
};
const GRID_DEFAULT = {
  left: 50,
  containLabel: true,
};
const Y_AXIS_AXIS_LABEL_DEFAULT = {
  showMinLabel: false,
  fontWeight: 700,
  fontSize: 12,
  color: "#21252D",
};
const Y_AXIS_DEFAULT = {
  nameLocation: "middle",
  nameRotate: 90,
  nameGap: 35,
  nameTextStyle: {
    ...AXIS_NAME_TEXT_STYLE,
  },
  axisLabel: {
    ...Y_AXIS_AXIS_LABEL_DEFAULT,
  },
};
const X_AXIS_LABEL_DEFAULT = {
  fontWeight: 700,
  fontSize: 12,
  color: "#21252D",
};
const X_AXIS_DEFAULT = {
  nameLocation: "middle",
  nameGap: 20,
  nameTextStyle: {
    ...AXIS_NAME_TEXT_STYLE,
  },
  axisLabel: {
    ...X_AXIS_LABEL_DEFAULT,
  },
};
const BAR_SERIES_DEFAULT = {
  barMaxWidth: 40,
};

function getBlockLimit() {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  return urlParams.get("limit") || 1000;
}

async function getBlockchainSummary() {
  return await fetch(config.rest_endpoint + "/summary", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
}
