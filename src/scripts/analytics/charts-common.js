const TOOLTIP_DEFAULT = { position: "top" };
const TITLE_DEFAULT = {
  left: 25,
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
  inside: true,
  showMinLabel: false,
};
const Y_AXIS_DEFAULT = {
  nameLocation: "middle",
  nameRotate: 90,
  nameGap: 20,
  axisLabel: {
    ...Y_AXIS_AXIS_LABEL_DEFAULT,
  },
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
