suite(["@CI"],'blocks table', () => {

    let pages = ['/blocks', '/summary', '/'];

    pages.forEach(page => it(`on ${page} includes canonical blocks according to selection`, () => {
        cy.visit(page);
        cy.get('select#canonical-selection').as('menu');
        
        cy.get('@menu').select('Canonical');
        cy.url().should('include', 'canonical=true');
        cy.wait(500);
        cy.contains('section','Blocks').find('table tr:not(:has(th)) span', {timeout: 60000}).as('tableRows')
        cy.get('@tableRows', {timeout: 20000}).should('not.have.class', 'bg-status-failed');
        cy.get('@tableRows', {timeout: 20000}).should('have.class', 'bg-status-success');
        
        cy.wait(500);
        cy.get('@menu').select('Non-Canonical');
        cy.wait(500);
        cy.url().should('include', 'canonical=false');
        cy.get('@tableRows', {timeout: 20000}).should('not.have.class', 'bg-status-success');
        cy.get('@tableRows', {timeout: 20000}).should('have.class', 'bg-status-failed');

        cy.wait(500);
        cy.get('@menu').select('All');
        cy.wait(500);
        cy.url().should('not.include', 'canonical');
        cy.get('@tableRows', {timeout: 20000}).should('have.class', 'bg-status-success');
        cy.get('@tableRows', {timeout: 20000}).should('have.class', 'bg-status-failed');
    }));

})