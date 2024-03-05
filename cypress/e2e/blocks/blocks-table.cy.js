suite(["@CI"],'blocks table', () => {

    let pages = ['/blocks', '/summary', '/'];
    let columns = ['Height', 'State Hash', 'Slot', 'Age', 'Block Producer', 'Coinbase', 'Transactions', 'SNARKs', 'Coinbase Receiver'];

    pages.forEach(page => it(`on ${page} includes non-canonical blocks when toggled is checked`, () => {
        cy.visit(page);
        cy.get('label').contains('Include Non-Canonical').as('toggle');
        cy.get('@toggle').click();
        cy.url().should('include', 'include_non_canonical=true');
        cy.wait(500);
        cy.contains('section','Blocks').find('table tr:not(:has(th)) span', {timeout: 60000}).as('tableRows')
        cy.get('@tableRows').should('have.class', 'bg-status-failed');
        cy.get('@tableRows').should('have.class', 'bg-status-success');
        cy.get('@toggle').click();
        cy.wait(500);
        cy.url().should('include', 'include_non_canonical=false');
        cy.get('@tableRows').should('have.class', 'bg-status-success')
        cy.get('@tableRows').should('not.have.class', 'bg-status-failed')
    }));

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('Blocks', columns);
    }));

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('Blocks', columns);
    }));
})