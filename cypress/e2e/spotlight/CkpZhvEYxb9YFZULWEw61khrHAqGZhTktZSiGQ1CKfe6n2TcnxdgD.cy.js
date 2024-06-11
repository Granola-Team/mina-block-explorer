suite(
  ["@tier1"],
  "Command CkpZhvEYxb9YFZULWEw61khrHAqGZhTktZSiGQ1CKfe6n2TcnxdgD",
  () => {
    it("is canonical", () => {
      cy.visit(
        "/commands/user?q-txn-hash=CkpZhvEYxb9YFZULWEw61khrHAqGZhTktZSiGQ1CKfe6n2TcnxdgD",
      );
      cy.clickLinkInTable(0, "Txn Hash", "User Commands", "h1");
      cy.aliasTransposedTableRows("Command Spotlight", "canonical");
      cy.get("@canonical").find("td").should("contain", "true");
    });
  },
);
