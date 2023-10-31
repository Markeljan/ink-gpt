// Copyright 2022 @paritytech/contracts-ui authors & contributors
// SPDX-License-Identifier: GPL-3.0-only

import { DocumentAddIcon, CollectionIcon, CodeIcon } from '@heroicons/react/outline';
import { NavLink } from './NavLink';

export function Navigation() {
  return (
    <div className="navigation">
      <NavLink icon={CodeIcon} to={`/ink-gpt`}>
        Generate Contract
      </NavLink>
      <NavLink icon={DocumentAddIcon} to={`/add-contract`}>
        Add New Contract
      </NavLink>
      <NavLink end icon={CollectionIcon} to={`/`}>
        All Contracts
      </NavLink>
    </div>
  );
}
