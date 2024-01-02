
const pages = require('../../pages.json');

describe('desktop menu look and feel', () => {
    pages.forEach(page => {
        it(`captures desktop menu on the ${page} page`, () => {
            cy.visit(page);
            cy.get('header').matchImageSnapshot(`${page}-header`);
        });
    });

    it('has a header',() => {
        cy.visit("/");
        cy.get('header').matchImageSnapshot(`homepage-header`);
    })
    
  })