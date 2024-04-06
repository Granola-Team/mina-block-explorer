suite(["@CI"],'homepage',() => {
 
    
    it(`links to the granola site`, () => {
        cy.visit('/');
        cy.get('span')
            .contains('Powered by Granola')
            .get('span > a[href="https://granola.team"]')
    });

})