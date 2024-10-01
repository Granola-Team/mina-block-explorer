const SLOT_GROUPING = 50; // groups slots by 50
const CHART_COLORS = ["#7BBBCA", "#A57B66", "#E2918F", "#629EDE", "#84BD7C"];
const TOOLTIP_DEFAULT = { position: "top" };
const GRID_LINES = {
  show: true, // Show or hide grid lines for the X-axis
  lineStyle: {
    color: "#A6A6A6", // Color of the grid lines
    width: 1, // Width of the grid lines
    type: "dashed", // Style of the grid lines ('solid', 'dashed', 'dotted')
  },
};
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
  splitLine: { ...GRID_LINES },
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
  splitLine: { ...GRID_LINES },
};
const BAR_SERIES_DEFAULT = {
  barMaxWidth: 40,
};
const SERIES_LINE_AREA_STYLES = { opacity: 0.5 };

function getBlockLimit() {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  return urlParams.get("limit") || 1000;
}

async function getBlockchainSummary() {
  let response = await fetch(config.rest_endpoint + "/summary", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  return await response.json();
}

function scaleMina(nanomina) {
  const mina = nanomina / 1e9; // Convert nanomina to mina

  // If the value is less than 1 Mina, show it in nanomina
  if (nanomina < 1e9) {
    return `${nanomina} nanomina`; // Return the value in nanomina
  }

  // If the value is equal to or greater than 1 million Mina, use 'M'
  if (mina >= 1e6) {
    return `${(mina / 1e6).toFixed(2)}M Mina`; // Mega Mina (millions of Mina)
  }

  // If the value is equal to or greater than 1 thousand Mina, use 'k'
  if (mina >= 1e3) {
    return `${(mina / 1e3).toFixed(2)}k Mina`; // Kilo Mina (thousands of Mina)
  }

  // Otherwise, just show the value in Mina
  return `${mina.toFixed(2)} Mina`;
}

const nanominaToKMina = (value) => `${(value / 1e12).toFixed(0)}k`;

if (typeof module !== "undefined" && module.exports) {
  module.exports = {
    nanominaToKMina,
    scaleMina,
  };
}
