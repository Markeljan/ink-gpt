// Copyright 2022 @paritytech/contracts-ui authors & contributors
// SPDX-License-Identifier: GPL-3.0-only

import { HelpBox, Statistics } from '../components/homepage';
import { RootLayout } from 'ui/layout';
import { Chat } from 'ui/components/ink-gpt/chat';

export function InkGPT() {
    return (
        <RootLayout
            aside={
                <>
                    <HelpBox />
                    <Statistics />
                </>
            }
            heading="Contracts"
        >
            <Chat />
        </RootLayout>
    );
}
