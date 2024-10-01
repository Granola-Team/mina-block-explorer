const { nanominaToKMina } = require("../scripts/analytics/charts-common.js");

test("nanominaToKMina", () => {
  expect(nanominaToKMina(1e12)).toBe("1k");
  expect(nanominaToKMina(12345678901234)).toBe("12k");
});

const getOrdinal = (number) => {
  const suffixes = ["th", "st", "nd", "rd"];
  const v = number % 100;
  const suffix = suffixes[(v - 20) % 10] || suffixes[v] || suffixes[0];

  return number + suffix;
};

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

  test('should return "5th" for 5', () => {
    expect(getOrdinal(5)).toBe("5th");
  });

  test('should return "6th" for 6', () => {
    expect(getOrdinal(6)).toBe("6th");
  });

  test('should return "7th" for 7', () => {
    expect(getOrdinal(7)).toBe("7th");
  });

  test('should return "8th" for 8', () => {
    expect(getOrdinal(8)).toBe("8th");
  });

  test('should return "9th" for 9', () => {
    expect(getOrdinal(9)).toBe("9th");
  });

  test('should return "10th" for 10', () => {
    expect(getOrdinal(10)).toBe("10th");
  });
});
