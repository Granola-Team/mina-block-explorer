const {
  nanominaToKMina,
  scaleMina,
  getOrdinal,
  buildTree,
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
      canonical: true,
      stateHash: "3NLhfsN1QPHsKzqu6RaVyLKmMCYTXnkrueMoWpmRQErfBkh6v6as",
      previousStateHash: "3NLjanAoyqjqmPsQHafJcvQiGW2xbvyXANHxEyNwPwan2eUoZBV9",
      blockHeight: 10000,
    },
    {
      canonical: false,
      stateHash: "3NLB1hQ6wdSHQbDB1L1xYMUv6oARhWz25uCMg4oeyW1jjDrs5Zjx",
      previousStateHash: "3NLjanAoyqjqmPsQHafJcvQiGW2xbvyXANHxEyNwPwan2eUoZBV9",
      blockHeight: 10000,
    },
    {
      canonical: true,
      stateHash: "3NLjanAoyqjqmPsQHafJcvQiGW2xbvyXANHxEyNwPwan2eUoZBV9",
      previousStateHash: "3NKFhMmNeGZR8daKD8zWASLzUUzhVtyMp3DfjFteUxyKKAvukYwk",
      blockHeight: 9999,
    },
    {
      canonical: true,
      stateHash: "3NKFhMmNeGZR8daKD8zWASLzUUzhVtyMp3DfjFteUxyKKAvukYwk",
      previousStateHash: "3NLpcWsFesKbesARzHWFbbzQLDNGQP1Jto7MPPYwTrisRW935Agi",
      blockHeight: 9998,
    },
    {
      canonical: false,
      stateHash: "3NKrd5saMRPLRLXBZoN7wfxbwNhJVB5t2ksK4dVXatCKhYC1CK7o",
      previousStateHash: "3NLpcWsFesKbesARzHWFbbzQLDNGQP1Jto7MPPYwTrisRW935Agi",
      blockHeight: 9998,
    },
    {
      canonical: true,
      stateHash: "3NLpcWsFesKbesARzHWFbbzQLDNGQP1Jto7MPPYwTrisRW935Agi",
      previousStateHash: "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      blockHeight: 9997,
    },
    {
      canonical: false,
      stateHash: "3NLmHHjWp7vGwrMFrsYYYW3MJLBaRneSZLohGHhsAvGgEH1J4RbE",
      previousStateHash: "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      blockHeight: 9997,
    },
    {
      canonical: false,
      stateHash: "3NLsNLbpJGYkGYWVMPUfRoxkdi7PDrtpJRJBAhh94qtpLJ5FwRNK",
      previousStateHash: "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      blockHeight: 9997,
    },
    {
      canonical: false,
      stateHash: "3NLNWxa9GWpcAn1NF6ULPVifBm77PTWkmBsPvaQA9EZQNftJYrd6",
      previousStateHash: "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      blockHeight: 9997,
    },
    {
      canonical: true,
      stateHash: "3NKeCAcqyEUEjenRvFVuea58wmSKuLLCvbtTFaRMjPqkM9Axs2dw",
      previousStateHash: "3NLoTex5WS72VpQkqZqVTkgxTbMq4Gm6ABLebiju158dZTB5jrFW",
      blockHeight: 9996,
    },
  ];

  test("Build tree from the lowest block height", () => {
    const tree = buildTree(inputBlocks);
    expect(tree.blockHeight).toBe(9996); // Root should be the lowest block height

    // Check if the next level of children exists
    expect(tree.children).toHaveLength(4); // 3 non-canonical and one canonical
  });

  test("Ensure tree extends to blockHeight 10000", () => {
    const tree = buildTree(inputBlocks);
    console.log(JSON.stringify(tree));
    let currentBlock = tree;

    // Traverse the tree upwards by following the hash linkage
    while (currentBlock.children.length > 0) {
      currentBlock = currentBlock.children[0]; // Follow the first child
    }

    expect(currentBlock.blockHeight).toBe(10000);
    expect(currentBlock.stateHash).toBe(
      "3NLhfsN1QPHsKzqu6RaVyLKmMCYTXnkrueMoWpmRQErfBkh6v6as",
    );
  });
});
