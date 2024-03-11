suite(["@CI"],'table', () => {

    let pages = [{
            url:'/transactions',
            section: 'Transactions',
        },
        { 
            url: '/blocks',
            section: 'Blocks',
        }, {
            url: '/summary',
            section: 'Blocks',
        }, {
            url: '/',
            section: 'Blocks',
        }, ];

    pages.forEach(({url,section}) => it(`on ${url} includes canonical blocks according to selection`, () => {
        cy.visit(url);
        cy.get('select#canonical-selection').as('menu');
        
        cy.get('@menu').select('Canonical');
        cy.url().should('include', 'canonical=true');
        cy.wait(500);
        cy.contains('section',section).find('table tr:not(:has(th)) span', {timeout: 60000}).as('tableRows')
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