const {
  nanominaToKMina,
  scaleMina,
  getOrdinal,
  buildTree,
  getMaxDepth,
  getUnixTimestampTruncatedToDay,
  unixTimestampToDateString,
  dayAxisLabelFormatter,
} = require("../scripts/analytics/charts-common.js");

test("nanominaToKMina", () => {
  expect(nanominaToKMina(1e12)).toBe("1k");
  expect(nanominaToKMina(12345678901234)).toBe("12k");
});

describe("scaleMina", () => {
  test('should return "1 nanomina" for input 1', () => {
    expect(scaleMina(1)).toBe("1 nanomina");
  });

  test('should return "100000 nanomina" for input 100000', () => {
    expect(scaleMina(1e5)).toBe("100000 nanomina");
  });

  test('should return "1.00 Mina" for input 1e9', () => {
    expect(scaleMina(1e9)).toBe("1.00 Mina");
  });

  test('should return "5.00 Mina" for input 5e9', () => {
    expect(scaleMina(5e9)).toBe("5.00 Mina");
  });

  test('should return "1.00k Mina" for input 1e12', () => {
    expect(scaleMina(1e12)).toBe("1.00k Mina");
  });

  test('should return "1.00M Mina" for input 1e15', () => {
    expect(scaleMina(1e15)).toBe("1.00M Mina");
  });

  test('should return "12.35M Mina" for input 12345678901234', () => {
    expect(scaleMina(12345678901234654)).toBe("12.35M Mina");
  });

  test('should return "0 nanomina" for input 0', () => {
    expect(scaleMina(0)).toBe("0 nanomina");
  });
});

describe("getOrdinal", () => {
  test('should return "1st" for 1', () => {
    expect(getOrdinal(1)).toBe("1st");
  });

  test('should return "2nd" for 2', () => {
    expect(getOrdinal(2)).toBe("2nd");
  });

  test('should return "3rd" for 3', () => {
    expect(getOrdinal(3)).toBe("3rd");
  });

  test('should return "4th" for 4', () => {
    expect(getOrdinal(4)).toBe("4th");
  });

  test('should return "11th" for 11', () => {
    expect(getOrdinal(11)).toBe("11th");
  });

  test('should return "21st" for 21', () => {
    expect(getOrdinal(21)).toBe("21st");
  });

  test('should return "22nd" for 22', () => {
    expect(getOrdinal(22)).toBe("22nd");
  });

  test('should return "23rd" for 23', () => {
    expect(getOrdinal(23)).toBe("23rd");
  });

  test('should return "101st" for 101', () => {
    expect(getOrdinal(101)).toBe("101st");
  });

  test('should return "111th" for 111', () => {
    expect(getOrdinal(111)).toBe("111th");
  });

  test('should return "112th" for 112', () => {
    expect(getOrdinal(112)).toBe("112th");
  });

  test('should return "113th" for 113', () => {
    expect(getOrdinal(113)).toBe("113th");
  });
});

describe("Tree Building with Correct Data", () => {
  const inputBlocks = [
    {
      blockHeight: 10000,
      globalSlotSinceGenesis: 14106,
      transactions: { coinbase: "720000000000" },
      creator: "B62qqgGAQfpFhX8G1iF253C37CMsj6ypn77C9fr3Y17iU6B5Ft4XjPo",
      snarkFees: "0",
      txFees: "15000000",
      canonical: true,
      stateHash: "3NLhfsN1QPHsKzqu6RaVyLKmMCYTXnkrueMoWpmRQErfBkh6v6as",
      protocolState: {
        previousStateHash:
          "3NLjanAoyqjqmPsQHafJcvQiGW2xbvyXANHxEyNwPwan2eUoZBV9",
      },
    },
    {
      blockHeight: 10000,
      globalSlotSinceGenesis: 14106,
      transactions: { coinbase: "720000000000" },
      creator: "B62qoVopcNoQPFydweGWUBnJJbrokkebVDiWGmAzYoaLysrFfzNCbya",
      snarkFees: "0",
      txFees: "15000000",
      canonical: false,
      stateHash: "3NLB1hQ6wdSHQbDB1L1xYMUv6oARhWz25uCMg4oeyW1jjDrs5Zjx",
      protocolState: {
        previousStateHash:
          "3NLjanAoyqjqmPsQHafJcvQiGW2xbvyXANHxEyNwPwan2eUoZBV9",
      },
    },
    {
      blockHeight: 9999,
      globalSlotSinceGenesis: 14105,
      transactions: { coinbase: "1440000000000" },
      creator: "B62qqhURJQo3CvWC3WFo9LhUhtcaJWLBcJsaA3DXaU2GH5KgXujZiwB",
      snarkFees: "0",
      txFees: "12000000",
      canonical: true,
      stateHash: "3NLjanAoyqjqmPsQHafJcvQiGW2xbvyXANHxEyNwPwan2eUoZBV9",
      protocolState: {
        previousStateHash:
          "3NKrd5saMRPLRLXBZoN7wfxbwNhJVB5t2ksK4dVXatCKhYC1CK7o",
      },
    },
    {
      blockHeight: 9998,
      globalSlotSinceGenesis: 14219,
      transactions: { coinbase: "720000000000" },
      creator: "B62qoeKm4p9J6Q3hYWzb82Yo5uP163MqooBX4ZWjp8FpX3N6Y81QgFc",
      snarkFees: "0",
      txFees: "0",
      canonical: false,
      stateHash: "3NKFhMmNeGZR8daKD8zWASLzUUzhVtyMp3DfjFteUxyKKAvukYwk",
      protocolState: {
        previousStateHash:
          "3NLmHHjWp7vGwrMFrsYYYW3MJLBaRneSZLohGHhsAvGgEH1J4RbE",
      },
    },
    {
      blockHeight: 9998,
      globalSlotSinceGenesis: 14104,
      transactions: { coinbase: "720000000000" },
      creator: "B62qq2PGvsoNC4u3uxthhs2ztXzuXRzqoWX9pWUDo5xdC3vdG7DQkjU",
      snarkFees: "0",
      txFees: "25000000",
      canonical: true,
      stateHash: "3NKrd5saMRPLRLXBZoN7wfxbwNhJVB5t2ksK4dVXatCKhYC1CK7o",
      protocolState: {
        previousStateHash:
          "3NLmHHjWp7vGwrMFrsYYYW3MJLBaRneSZLohGHhsAvGgEH1J4RbE",
      },
    },
    {
      blockHeight: 9997,
      globalSlotSinceGenesis: 14102,
      transactions: { coinbase: "720000000000" },
      creator: "B62qqgGAQfpFhX8G1iF253C37CMsj6ypn77C9fr3Y17iU6B5Ft4XjPo",
      snarkFees: "0",
      txFees: "12000000",
      canonical: false,
      stateHash: "3NLpcWsFesKbesARzHWFbbzQLDNGQP1Jto7MPPYwTrisRW935Agi",
      protocolState: {
        previousStateHash:
          "3NKdioMLUnWMaAfCRSN7xDXGhYE4HFZXnncg7W4goigLCsGRK2oE",
      },
    },
    {
      blockHeight: 9997,
      globalSlotSinceGenesis: 14102,
      transactions: { coinbase: "720000000000" },
      creator: "B62qqgGAQfpFhX8G1iF253C37CMsj6ypn77C9fr3Y17iU6B5Ft4XjPo",
      snarkFees: "0",
      txFees: "12000000",
      canonical: false,
      stateHash: "3NLmHHjWp7vGwrMFrsYYYW3MJLBaRneSZLohGHhsAvGgEH1J4RbE",
      protocolState: {
        previousStateHash:
          "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      },
    },
    {
      blockHeight: 9997,
      globalSlotSinceGenesis: 14102,
      transactions: { coinbase: "720000000000" },
      creator: "B62qoeKm4p9J6Q3hYWzb82Yo5uP163MqooBX4ZWjp8FpX3N6Y81QgFc",
      snarkFees: "0",
      txFees: "12000000",
      canonical: false,
      stateHash: "3NLsNLbpJGYkGYWVMPUfRoxkdi7PDrtpJRJBAhh94qtpLJ5FwRNK",
      protocolState: {
        previousStateHash:
          "3NKdioMLUnWMaAfCRSN7xDXGhYE4HFZXnncg7W4goigLCsGRK2oE",
      },
    },
    {
      blockHeight: 9997,
      globalSlotSinceGenesis: 14102,
      transactions: { coinbase: "720000000000" },
      creator: "B62qoeKm4p9J6Q3hYWzb82Yo5uP163MqooBX4ZWjp8FpX3N6Y81QgFc",
      snarkFees: "0",
      txFees: "12000000",
      canonical: false,
      stateHash: "3NLNWxa9GWpcAn1NF6ULPVifBm77PTWkmBsPvaQA9EZQNftJYrd6",
      protocolState: {
        previousStateHash:
          "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      },
    },
    {
      blockHeight: 9996,
      globalSlotSinceGenesis: 14101,
      transactions: { coinbase: "720000000000" },
      creator: "B62qrQBarKiVK11xP943pMQxnmNrfYpT7hskHLWdFXbx2K1E9wR1Vdy",
      snarkFees: "0",
      txFees: "13000000",
      canonical: true,
      stateHash: "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      protocolState: {
        previousStateHash:
          "3NLoTex5WS72VpQkqZqVTkgxTbMq4Gm6ABLebiju158dZTB5jrFW",
      },
    },
    {
      blockHeight: 9996,
      globalSlotSinceGenesis: 14101,
      transactions: { coinbase: "720000000000" },
      creator: "B62qmFf6UZn2sg3j8bYLGmMinzS2FHX6hDM71nFxAfMhvh4hnGBtkBD",
      snarkFees: "0",
      txFees: "13000000",
      canonical: false,
      stateHash: "3NKdioMLUnWMaAfCRSN7xDXGhYE4HFZXnncg7W4goigLCsGRK2oE",
      protocolState: {
        previousStateHash:
          "3NLoTex5WS72VpQkqZqVTkgxTbMq4Gm6ABLebiju158dZTB5jrFW",
      },
    },
    {
      blockHeight: 9995,
      globalSlotSinceGenesis: 14100,
      transactions: { coinbase: "720000000000" },
      creator: "B62qj3Gzxgb4G4M8CwZRXZPtmVwGJtGfVXVbpMrACNDSqQLoXzSQ9HW",
      snarkFees: "0",
      txFees: "12000000",
      canonical: true,
      stateHash: "3NLoTex5WS72VpQkqZqVTkgxTbMq4Gm6ABLebiju158dZTB5jrFW",
      protocolState: {
        previousStateHash:
          "3NKiHSvFR4k5iNXLMjrdQm23gUcmTwivf4dC4Va7Rn3zjmjQXdgt",
      },
    },
    {
      blockHeight: 9994,
      globalSlotSinceGenesis: 14099,
      transactions: { coinbase: "720000000000" },
      creator: "B62qmRG3THXszPjfJXDCk2MjDZqWLXMoVzyEWMPStEdfqhMe7GJaGxE",
      snarkFees: "0",
      txFees: "12000000",
      canonical: true,
      stateHash: "3NKiHSvFR4k5iNXLMjrdQm23gUcmTwivf4dC4Va7Rn3zjmjQXdgt",
      protocolState: {
        previousStateHash:
          "3NKfdnW8uY2CgpsubUKT2K8kLXFFa4HhmkCgbmBQkKvXKcuNf41d",
      },
    },
    {
      blockHeight: 9994,
      globalSlotSinceGenesis: 14099,
      transactions: { coinbase: "720000000000" },
      creator: "B62qoXQhp63oNsLSN9Dy7wcF3PzLmdBnnin2rTnNWLbpgF7diABciU6",
      snarkFees: "0",
      txFees: "12000000",
      canonical: false,
      stateHash: "3NKnpRrkaoLtsJL7xdsMVMxFPdsMK3HwtdvYzsTg13wJ9vzyLiwQ",
      protocolState: {
        previousStateHash:
          "3NKfdnW8uY2CgpsubUKT2K8kLXFFa4HhmkCgbmBQkKvXKcuNf41d",
      },
    },
    {
      blockHeight: 9993,
      globalSlotSinceGenesis: 14098,
      transactions: { coinbase: "720000000000" },
      creator: "B62qjhiEXP45KEk8Fch4FnYJQ7UMMfiR3hq9ZeMUZ8ia3MbfEteSYDg",
      snarkFees: "0",
      txFees: "12000000",
      canonical: true,
      stateHash: "3NKfdnW8uY2CgpsubUKT2K8kLXFFa4HhmkCgbmBQkKvXKcuNf41d",
      protocolState: {
        previousStateHash:
          "3NLtVqfra25CqZGXeHF1HH5XthLYeqqLYvnt3Q2GLmviMyy5nBwo",
      },
    },
    {
      blockHeight: 9993,
      globalSlotSinceGenesis: 14151,
      transactions: { coinbase: "720000000000" },
      creator: "B62qjw2PdqLJYNTr6cNDXEmnc9FLNoaqVMHMqiTX7sqLYJRpd3mshFA",
      snarkFees: "0",
      txFees: "582000000",
      canonical: false,
      stateHash: "3NKnRumoD28URCCZ9iCy5MxMMi91w8UrLNhavGgYH5bBFkqeHahE",
      protocolState: {
        previousStateHash:
          "3NLWbWj5US2atTS38JkecFofgRN3BUGhwsWGtMMvD2qbAeKJtb6m",
      },
    },
  ];

  test("Build tree from the lowest block height", () => {
    const tree = buildTree(inputBlocks);
    expect(tree.blockHeight).toBe(9993); // Root should be the lowest block height

    // Check if the next level of children exists
    expect(tree.children).toHaveLength(2);
  });

  test("Ensure tree extends to blockHeight 10000", () => {
    const tree = buildTree(inputBlocks);
    let currentBlock = tree;

    // Traverse the tree upwards by following the hash linkage
    while (currentBlock.children.length > 0) {
      currentBlock = currentBlock.children.find((child) => {
        return (child.canonical = true);
      });
    }

    expect(currentBlock.blockHeight).toBe(10000);
    expect(currentBlock.stateHash).toBe(
      "3NLhfsN1QPHsKzqu6RaVyLKmMCYTXnkrueMoWpmRQErfBkh6v6as",
    );
  });

  test("Get maximum depth of the tree", () => {
    const tree = buildTree(inputBlocks);
    const maxDepth = getMaxDepth(tree);
    expect(maxDepth).toBe(8); // Based on the current tree structure
  });
});

describe("time utils", () => {
  test("formats a valid date string in YYYY-MM-DD format to human-readable form", () => {
    const value = "2024-06-04"; // Date in YYYY-MM-DD format
    const expected = "Jun 4, 2024"; // Expected output for en-US locale

    const result = dayAxisLabelFormatter(value);

    expect(result).toBe(expected);
  });

  test("formats another valid date string correctly", () => {
    const value = "2023-12-25"; // Date in YYYY-MM-DD format
    const expected = "Dec 25, 2023"; // Expected output for en-US locale

    const result = dayAxisLabelFormatter(value);

    expect(result).toBe(expected);
  });

  test("handles leap year date formatting correctly", () => {
    const value = "2024-02-29"; // Leap year date
    const expected = "Feb 29, 2024"; // Expected output for en-US locale

    const result = dayAxisLabelFormatter(value);

    expect(result).toBe(expected);
  });

  test("getUnixTimestampTruncatedToDay should return correct Unix timestamp truncated to the day", () => {
    const dateStr = "2024-06-04T08:51:00.000Z";
    const expectedTimestamp = 1717459200; // Unix timestamp for 2024-06-04 00:00:00 UTC

    const result = getUnixTimestampTruncatedToDay(dateStr);

    expect(result).toBe(expectedTimestamp);
  });

  test("getUnixTimestampTruncatedToDay should handle different time values correctly", () => {
    const dateStr = "2024-06-04T23:59:59.999Z";
    const expectedTimestamp = 1717459200; // Unix timestamp for 2024-06-04 00:00:00 UTC

    const result = getUnixTimestampTruncatedToDay(dateStr);

    expect(result).toBe(expectedTimestamp);
  });

  test("getUnixTimestampTruncatedToDay should handle leap years correctly", () => {
    const dateStr = "2024-02-29T15:30:00.000Z";
    const expectedTimestamp = 1709164800; // Unix timestamp for 2024-02-29 00:00:00 UTC (leap year)

    const result = getUnixTimestampTruncatedToDay(dateStr);

    expect(result).toBe(expectedTimestamp);
  });

  test("unixTimestampToDateString converts timestamp correctly for a regular date", () => {
    const timestamp = 1717459200; // Unix timestamp for 2024-06-04 00:00:00 UTC
    const expectedDateString = "2024-06-04";

    const result = unixTimestampToDateString(timestamp);

    expect(result).toBe(expectedDateString);
  });

  test("unixTimestampToDateString converts timestamp correctly for a leap year date", () => {
    const timestamp = 1709164800; // Unix timestamp for 2024-02-29 00:00:00 UTC (leap year)
    const expectedDateString = "2024-02-29";

    const result = unixTimestampToDateString(timestamp);

    expect(result).toBe(expectedDateString);
  });

  test("unixTimestampToDateString handles timestamp at the Unix epoch", () => {
    const timestamp = 0; // Unix timestamp for 1970-01-01 00:00:00 UTC
    const expectedDateString = "1970-01-01";

    const result = unixTimestampToDateString(timestamp);

    expect(result).toBe(expectedDateString);
  });

  test("unixTimestampToDateString handles timestamp before 1970 (negative Unix timestamp)", () => {
    const timestamp = -86400; // Unix timestamp for 1969-12-31 00:00:00 UTC
    const expectedDateString = "1969-12-31";

    const result = unixTimestampToDateString(timestamp);

    expect(result).toBe(expectedDateString);
  });

  test("unixTimestampToDateString handles timestamp with string input", () => {
    const timestamp = "1717459200"; // Unix timestamp as string
    const expectedDateString = "2024-06-04";

    const result = unixTimestampToDateString(timestamp);

    expect(result).toBe(expectedDateString);
  });
});
