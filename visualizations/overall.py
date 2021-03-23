import pandas as pd
import plotly.graph_objects as go

# 'Reference': [
#     'Python/pyca',
#     'Python/pynacl',
#     'C/ed25519 (orlp)',
#     'C/ed25519-donna',
#     'Rust/ed25519-fun',
#     'Rust/ed25519-dalek',
#     'Rust/ed25519-dalek SIMD',
#     'C/ed25519-donna SSE2',
#     'Java/ed25519-java',
#     'Java/Bouncy Castle',
#     'Go/crypto',
#     'Go/ed25519 (oasis)',
# ],

# (overall)
key_gen = {
    'title': 'Key Generation',
    'Execution Time (μs)': [
        85.588,
        43.892,
        39.696,
        37.009,
        32.408,
        20.566,
        19.663,
        17.372,
        15.206,
        14.068,
        13.397,
        10.936,
    ],
    'Implementation': [
        'Java/ed25519-java',
        'Go/crypto',
        'Java/Bouncy Castle',
        'Python/pyca',
        'C/ed25519 (orlp)',
        'Python/pynacl',
        'C/ed25519-donna',
        'Rust/ed25519-fun',
        'Rust/ed25519-dalek',
        'Go/ed25519 (oasis)',
        'Rust/ed25519-dalek SIMD',
        'C/ed25519-donna SSE2',
    ],
}

# (overall)
sign_gen = {
    'title': 'Signature Generation',
    'Execution Time (μs)': [
        75.771,
        44.145,
        38.673,
        35.632,
        33.021,
        21.481,
        20.697,
        20.573,
        16.752,
        14.921,
        14.190,
        11.652,
    ],
    'Implementation': [
        'Java/ed25519-java',
        'Go/crypto',
        'Java/Bouncy Castle',
        'Python/pyca',
        'C/ed25519 (orlp)',
        'Rust/ed25519-fun',
        'C/ed25519-donna',
        'Python/pynacl',
        'Rust/ed25519-dalek',
        'Go/ed25519 (oasis)',
        'Rust/ed25519-dalek SIMD',
        'C/ed25519-donna SSE2',
    ]
}

# (overall)
sign_ver = {
    'title': 'Signature Verification',
    'Execution Time (μs)': [
        158.192,
        121.536,
        116.556,
        90.535,
        88.706,
        66.332,
        57.714,
        54.141,
        46.349,
        41.848,
        38.604,
        26.389,
    ],
    'Implementation': [
        'Java/ed25519-java',
        'Go/crypto',
        'Java/Bouncy Castle',
        'Python/pyca',
        'C/ed25519 (orlp)',
        'C/ed25519-donna',
        'Go/ed25519 (oasis)',
        'Rust/ed25519-fun',
        'Python/pynacl',
        'Rust/ed25519-dalek',
        'C/ed25519-donna SSE2',
        'Rust/ed25519-dalek SIMD',
    ]
}

lang_color = {
    'our': '#C5150C',
    'others': '#13112D',
}
languages = [
    'others',
    'others',
    'others',
    'others',
    'others',
    'our',
    'others',
    'others',
    'others',
    'others',
    'others',
    'others'
]
colors = [lang_color[x] for x in languages]

# key_gen, sign_gen, sign_ver
target = sign_gen

df = pd.DataFrame(
    target, columns=['Execution Time (μs)', 'Implementation'])

fig = go.Figure(data=go.Bar(y=df['Implementation'],
                            x=df['Execution Time (μs)'],
                            text=df['Execution Time (μs)'],
                            marker={'color': colors},
                            orientation='h',
                            )
                )

fig.update_layout(font_family="Courier New", font_size=20,
                  title=target['title'], template='plotly_white',
                  bargap=0.4,
                  xaxis_title="Execution Time (μs)",
                  yaxis_title="Implementation")

fig.update_traces(textposition='outside')

fig.show()
