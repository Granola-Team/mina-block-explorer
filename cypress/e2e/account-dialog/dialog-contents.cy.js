suite(["@CI"],'dialog',()=>{

    beforeEach(() => {
        cy.visit('/summary/accounts/B62qq3TQ8AP7MFYPVtMx5tZGF3kWLJukfwG1A1RGvaBW1jfTPTkDBW6');
    })
    
    it(`has correct sections`, () => {
        ["Transactions", "SNARK Jobs", "Block Production"].forEach(section => {
            cy.get('section h2', {timeout:60000})
                .contains(section, {timeout:60000})
        });
    });
    
})