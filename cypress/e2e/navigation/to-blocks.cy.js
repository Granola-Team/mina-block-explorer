import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"],'block page', () => {
    [{
        origin: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`,
        dest:"blocks",
        href:`/blocks?account=${DEFAULT_ACCOUNT_PK}`
    }].forEach(({origin, dest, href}) => it(`is navigated to from ${origin}`,() => {
        cy.visit(origin);
        cy.get('a').contains("See all block production", {timeout: 60000}).click();
        cy.url().should('contain', href);
    }));
});