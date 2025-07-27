/**
 * @vitest-environment jsdom
 */

import React from 'react';
import { render, screen } from '@testing-library/react';
import { test, expect } from 'vitest';

// Simple component to test
function SimpleButton({ label }) {
  return <button>{label}</button>;
}

test('renders a button with the correct label', () => {
  render(<SimpleButton label="Click me" />);
  const button = screen.getByText('Click me');
  expect(button).toBeInTheDocument();
});
