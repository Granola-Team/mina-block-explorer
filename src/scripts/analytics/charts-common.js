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

function getUrlParam(param) {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  return urlParams.get(param);
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

function getOrdinal(number) {
  const suffixes = ["th", "st", "nd", "rd"];
  const v = number % 100;
  const suffix = suffixes[(v - 20) % 10] || suffixes[v] || suffixes[0];

  return number + suffix;
}
// Group blocks by blockHeight
function groupBlocksByHeight(blocks) {
  return blocks.reduce((acc, block) => {
    if (!acc[block.blockHeight]) {
      acc[block.blockHeight] = [];
    }
    acc[block.blockHeight].push(block);
    return acc;
  }, {});
}

function buildTree(blocks) {
  const blocksByHeight = groupBlocksByHeight(blocks);

  // Find the root block with the lowest height
  const rootHeight = Math.min(...Object.keys(blocksByHeight));
  const root = blocksByHeight[rootHeight].find((block) => block.canonical); // Start with the canonical block at the root

  const visited = new Set(); // Track visited blocks to avoid duplicates
  const queue = [root]; // Initialize the queue with the root node

  visited.add(root.stateHash); // Mark the root as visited
  root.children = []; // Initialize the children for the root

  while (queue.length > 0) {
    const node = queue.shift(); // Dequeue the next node

    const nextHeight = node.blockHeight + 1;

    // Find the canonical block at the next height whose previousStateHash matches the current node's stateHash
    const canonicalChild = blocksByHeight[nextHeight]
      ? blocksByHeight[nextHeight].find(
          (block) =>
            block.protocolState.previousStateHash === node.stateHash &&
            block.canonical,
        )
      : null;

    if (canonicalChild && !visited.has(canonicalChild.stateHash)) {
      visited.add(canonicalChild.stateHash); // Mark the canonical child as visited
      canonicalChild.children = []; // Initialize the children
      node.children.push(canonicalChild); // Attach the canonical child
      queue.push(canonicalChild); // Add the canonical child to the queue
    }

    // Now add non-canonical children at the next height whose previousStateHash matches the current node's stateHash
    const nonCanonicalChildren = blocksByHeight[nextHeight]
      ? blocksByHeight[nextHeight].filter(
          (block) =>
            block.protocolState.previousStateHash === node.stateHash &&
            !block.canonical,
        )
      : [];

    nonCanonicalChildren.forEach((child) => {
      if (!visited.has(child.stateHash)) {
        visited.add(child.stateHash); // Mark non-canonical child as visited
        child.children = []; // Initialize the children for the child
        node.children.push(child); // Attach the non-canonical child
        queue.push(child); // Add to the queue
      }
    });
  }

  return root;
}

function mapTreeToEchartsFormat(node) {
  // Base case: if the node has no children, return it with the required changes
  const formattedNode = {
    name: node.blockHeight.toString(), // Map blockHeight to name (as a string)
    ...node,
    children: node.children.map(mapTreeToEchartsFormat), // Recursively format the children
    previousStateHash: node.previousStateHash,
  };

  return formattedNode;
}

if (typeof module !== "undefined" && module.exports) {
  module.exports = {
    nanominaToKMina,
    scaleMina,
    getOrdinal,
    buildTree,
  };
}
