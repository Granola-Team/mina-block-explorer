suite(["@CI"],'tab counts', () => {

    let page = '/blocks/3NKxoDDgKH9Je4pg86wNgkK3LqnSY1A4KANzWrAvmUn74cWRENmP/spotlight';

    let tabs = [
        "SNARK Jobs",
        "User Commands",
        "Fee Transfers"
    ];

    tabs.forEach(tab => it(`match to table pagination counts on ${tab} tab`,() => {
        cy.visit(page);
        cy.get('a').contains(tab).as('tab');
        cy.get('@tab').siblings('.number-bubble').first().as('tab-count');

        cy.get('@tab').click();        
        cy.get('@tab-count').invoke('text').then((count) => {
            cy.scrollTo('bottom');
            cy.get('.pagination-controls').children().first().contains(count);
        });
    }));

});
