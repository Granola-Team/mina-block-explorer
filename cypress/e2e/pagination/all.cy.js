suite(["@CI"],'pagination',() => {

    ['/','/summary','/blocks','/transactions','/snarks','/stakes','/next-stakes'].forEach(page => it(`works on ${page}`,() => {
        cy.visit(page);
        cy.get('.pagination-controls', {timeout: 30000}).find('button').last().as('next');
        cy.get('.pagination-controls', {timeout: 30000}).find('button').first().as('prev');
        cy.get('.pagination-controls', {timeout: 30000}).find('.current-page').as('currentPage');

        // initial check
        cy.get('@prev').should('be.disabled');
        cy.get('@next').should('not.be.disabled');
        cy.get('@currentPage').should('contain',1);

        // next page
        cy.get('@next').click();
        cy.wait(1000);
        cy.get('@prev').should('not.be.disabled');
        cy.get('@next').should('not.be.disabled');
        cy.get('@currentPage').should('contain',2);    

        // number click (last page)
        cy.get('.pagination-controls', {timeout: 30000}).find('button').contains('5').click();
        cy.wait(1000);
        cy.get('@prev').should('not.be.disabled');
        cy.get('@next').should('be.disabled');
        cy.get('@currentPage').should('contain',5);    

        // prev page
        cy.get('@prev').click();
        cy.wait(1000);
        cy.get('@prev').should('not.be.disabled');
        cy.get('@next').should('not.be.disabled');
        cy.get('@currentPage').should('contain',4);    

    }));

});