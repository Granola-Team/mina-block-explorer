suite(["@CI"],'Account spotlight', () => {

    let expected_fields = [
        'Balance', 'Receipt Chain Hash', 'Voting For', 'Delegate'
    ];
    let account_id = "B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6";

    it('displays complete information', () => {
        cy.visit(`/addresses/accounts/${account_id}`);
        cy.testSpotlight("Account Spotlight", account_id, expected_fields);
    });
});