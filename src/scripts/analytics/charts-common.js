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

function getUrlParamOrDefault(key, defaultVal) {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  if (urlParams.has(key)) {
    return urlParams.get(key);
  } else {
    return defaultVal;
  }
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
  const queue = [{ node: root, shouldAlternateLeft: true }]; // Initialize the queue with the root node and an alternating flag

  visited.add(root.stateHash); // Mark the root as visited
  root.children = []; // Initialize the children for the root

  while (queue.length > 0) {
    const { node, shouldAlternateLeft } = queue.shift(); // Dequeue the next node and flag

    const nextHeight = node.blockHeight + 1;

    // Find the canonical block at the next height whose previousStateHash matches the current node's stateHash
    const canonicalChild = blocksByHeight[nextHeight]
      ? blocksByHeight[nextHeight].find(
          (block) =>
            block.protocolState.previousStateHash === node.stateHash &&
            block.canonical,
        )
      : null;

    // Now find non-canonical children
    const nonCanonicalChildren = blocksByHeight[nextHeight]
      ? blocksByHeight[nextHeight].filter(
          (block) =>
            block.protocolState.previousStateHash === node.stateHash &&
            !block.canonical,
        )
      : [];

    // Combine the canonical and non-canonical children to count the total
    let allChildren = nonCanonicalChildren;
    if (canonicalChild) {
      const totalChildrenCount = nonCanonicalChildren.length + 1; // Total count including canonical child
      const middleIndex = Math.floor(nonCanonicalChildren.length / 2);

      // Adjust the insertion index based on total children being even or odd, and the alternating flag
      const insertIndex =
        totalChildrenCount % 2 === 0
          ? shouldAlternateLeft
            ? middleIndex
            : middleIndex + 1
          : middleIndex;

      // Create the full list with the canonical child inserted at the computed index
      allChildren = [
        ...nonCanonicalChildren.slice(0, insertIndex),
        canonicalChild,
        ...nonCanonicalChildren.slice(insertIndex),
      ];
    }

    // Add the children to the current node
    allChildren.forEach((child) => {
      if (!visited.has(child.stateHash)) {
        visited.add(child.stateHash); // Mark as visited
        child.children = []; // Initialize children for the child
        node.children.push(child); // Attach the child to the current node
        // Alternate the insertion position for even numbered children
        queue.push({ node: child, shouldAlternateLeft: !shouldAlternateLeft });
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
    itemStyle: { color: node.canonical ? "#56D05F" : "#FB7631" },
    children: node.children.map(mapTreeToEchartsFormat), // Recursively format the children
    previousStateHash: node.previousStateHash,
  };

  return formattedNode;
}

function getMaxDepth(node) {
  if (!node.children || node.children.length === 0) {
    return 1; // A leaf node has a depth of 1
  }

  // Recursively calculate the depth of each child, and return the maximum depth + 1 (for the current node)
  const childDepths = node.children.map(getMaxDepth);
  return Math.max(...childDepths) + 1;
}

function getUnixTimestampTruncatedToDay(dateString) {
  const date = new Date(dateString);
  // Set hours, minutes, seconds, and milliseconds to 0
  date.setUTCHours(0, 0, 0, 0);
  // Return the Unix timestamp (seconds since 1970-01-01)
  return Math.floor(date.getTime() / 1000);
}

function unixTimestampToDateString(timestamp) {
  return new Date(parseInt(timestamp, 10) * 1000)
    .toISOString()
    .substring(0, 10);
}

function dayAxisLabelFormatter(value) {
  const parts = value.split("-");
  const date = new Date(Date.UTC(parts[0], parts[1] - 1, parts[2])); // Treat as UTC

  const year = date.getUTCFullYear();
  const month = date.toLocaleString("en-US", {
    month: "short",
    timeZone: "UTC",
  });
  const day = date.getUTCDate(); // No zero-padding for single-digit days

  return `${month} ${day}, ${year}`;
}

if (typeof module !== "undefined" && module.exports) {
  module.exports = {
    nanominaToKMina,
    scaleMina,
    getOrdinal,
    buildTree,
    getMaxDepth,
    getUnixTimestampTruncatedToDay,
    unixTimestampToDateString,
    dayAxisLabelFormatter,
  };
}
