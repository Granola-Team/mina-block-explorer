
const pages = require('../../pages.json');

describe('desktop menu look and feel', () => {
    pages.forEach(page => {
        it(`opens and closes the desktop menu on the ${page} page`, () => {
            cy.visit(page);
            cy.get('nav').screenshot();
        });
    });

    it('has a header',() => {
        cy.visit("/");
        cy.get('header').screenshot();
    })
    
  })