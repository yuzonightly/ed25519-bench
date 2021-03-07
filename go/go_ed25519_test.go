// Copyright (c) 2016 The Go Authors. All rights reserved.
// Copyright (c) 2019 Oasis Labs Inc.  All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//   * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.s

package benchmarks

import (
	crypto "crypto/ed25519"
	"testing"

	oasis "github.com/oasisprotocol/ed25519"
)

type zeroReader struct{}

func (zeroReader) Read(buf []byte) (int, error) {
	for i := range buf {
		buf[i] = 0
	}
	return len(buf), nil
}

func BenchmarkGoCryptoKeyGeneration(b *testing.B) {
	var zero zeroReader
	for i := 0; i < b.N; i++ {
		if _, _, err := crypto.GenerateKey(zero); err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkGoCryptoSignatureGeneration(b *testing.B) {
	var zero zeroReader
	_, priv, err := crypto.GenerateKey(zero)
	if err != nil {
		b.Fatal(err)
	}
	message := []byte("")
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		crypto.Sign(priv, message)
	}
}

func BenchmarkGoCryptoSignatureVerification(b *testing.B) {
	var zero zeroReader
	pub, priv, err := crypto.GenerateKey(zero)
	if err != nil {
		b.Fatal(err)
	}
	message := []byte("")
	signature := crypto.Sign(priv, message)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		crypto.Verify(pub, message, signature)
	}
}

func BenchmarkOasisKeyGeneration(b *testing.B) {
	var zero zeroReader
	for i := 0; i < b.N; i++ {
		if _, _, err := oasis.GenerateKey(zero); err != nil {
			b.Fatal(err)
		}
	}
}

func BenchmarkOasisSignatureGeneration(b *testing.B) {
	var zero zeroReader
	_, priv, err := oasis.GenerateKey(zero)
	if err != nil {
		b.Fatal(err)
	}
	message := []byte("")
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		oasis.Sign(priv, message)
	}
}

func BenchmarkOasisSignatureVerification(b *testing.B) {
	var zero zeroReader
	pub, priv, err := oasis.GenerateKey(zero)
	if err != nil {
		b.Fatal(err)
	}
	message := []byte("")
	signature := oasis.Sign(priv, message)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		oasis.Verify(pub, message, signature)
	}
}
