import { GENESIS_BLOCK_BLOCK_HASH } from "../constants";

suite(["@tier1"], "<head>", () => {
  let sample_non_charting_pages = ["/", "/blocks"];
  let charting_pages = [`/blocks/${GENESIS_BLOCK_BLOCK_HASH}/analytics`];

  sample_non_charting_pages.forEach((page) =>
    it(`does not contain charting libraries on non-charting page ${page}`, () => {
      cy.visit(page);
      cy.wait(2000); //wait for header to update
      cy.get("head").then((head) => {
        let headToText = JSON.stringify(head.html());
        expect(headToText).to.not.contain("echarts");
      });
    }),
  );

  charting_pages.forEach((page) =>
    it(`contains charting libraries on ${page}`, () => {
      cy.visit(page);
      cy.wait(2000); //wait for header to update
      cy.get("head").then((head) => {
        let headToText = JSON.stringify(head.html());
        expect(headToText).to.contain("echarts");
      });
    }),
  );
});
