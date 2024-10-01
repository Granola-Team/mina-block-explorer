const {
  nanominaToKMina,
  scaleMina,
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
