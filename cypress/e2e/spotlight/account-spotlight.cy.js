import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"],'Account spotlight', () => {

    let expected_fields = [
        'Balance', 'Voting For', 'Delegate'
    ];

    it('displays complete information', () => {
        cy.visit(`/addresses/accounts/${DEFAULT_ACCOUNT_PK}`);
        cy.testSpotlight("Account Spotlight", DEFAULT_ACCOUNT_PK, expected_fields);
    });
});