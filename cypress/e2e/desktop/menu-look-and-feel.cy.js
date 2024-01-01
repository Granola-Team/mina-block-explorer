
const pages = require('../../pages.json');

describe('desktop menu look and feel', () => {
    pages.forEach(page => {
        it(`captures desktop menu on the ${page} page`, () => {
            cy.visit(page);
            cy.get('header').screenshot();
        });
    });

    it('has a header',() => {
        cy.visit("/");
        cy.get('header').screenshot();
    })
    
  })