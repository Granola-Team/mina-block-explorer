// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })

Cypress.Commands.add('closeAccountDialog', () => {
  cy.get('dialog button#closedialog a').click();
  cy.get('dialog').should('not.exist');
});

Cypress.Commands.add('openAccountDialog', (nthRow, columnHeading, tableHeading) => {
  cy.clickLinkInTable(nthRow, columnHeading, tableHeading);
  cy.get('dialog', { timeout: 10000 }).should('be.visible');
});

Cypress.Commands.add('accountDialogToAccount', () => {
  cy.get('dialog button#viewmore a').click();
  cy.get('dialog').should('not.exist');

  cy.url().should('contain', '/accounts')
});

Cypress.Commands.add('openMobileMenu', () => {
  cy.get('nav').should('not.be.visible');
  cy.get('label[for="nav-toggle"]').click();
  cy.get('nav').should('be.visible');
});

Cypress.Commands.add('clickLinkInTable', (nthRow, columnHeading, tableHeading) => {
  cy.get('h1')
    .contains(tableHeading)
    .parent()
    .find('table th').contains(columnHeading, {timeout: 30000}) 
    .invoke('index')
    .then(columnIndex => {
      cy.get('h1')
        .contains(tableHeading)
        .parent()
        .find('table tr', {timeout: 30000}) 
        .eq(nthRow) 
        .find('td')
        .eq(columnIndex)
        .find('a') 
        .click({force: true});            
    });
})


