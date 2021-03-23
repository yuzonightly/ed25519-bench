import plotly.graph_objects as go

# (architecture)
key_gen = {
    'title': 'Key Generation',
    '32': [
        47.567,
        66.744,
        28.897,
    ],
    '64': [
        20.444,
        31.567,
        15.370,
    ],
}

# (architecture)
sign_gen = {
    'title': 'Signature Generation',
    '32': [
        49.153,
        68.554,
        29.710,
    ],
    '64': [
        20.693,
        32.136,
        16.202,
    ],
}

# (architecture)
sign_ver = {
    'title': 'Signature Verification',
    '32': [
        148.740,
        188.096,
        82.116,
    ],
    '64': [
        67.071,
        87.093,
        42.711,
    ],
}

# key_gen, sign_gen, sign_ver
target = sign_ver

fig = go.Figure()

colors32 = ['#C5150C', '#C5150C', '#C5150C']
colors64 = ['#13112D', '#13112D', '#13112D']

fig.add_trace(go.Bar(
    name='32-bit',
    text=target['32'],
    marker={'color': colors32},
    x=['ed25519-donna', 'ed25519 (orlp)', 'ed25519-dalek'], y=target['32'],
))

fig.add_trace(go.Bar(
    name='64-bit',
    text=target['64'],
    marker={'color': colors64},
    x=['ed25519-donna', 'ed25519 (orlp)', 'ed25519-dalek'], y=target['64'],
))

fig.update_layout(barmode='group', font_family='Courier New',
                  title=target['title'], legend_title='Architecture',
                  template='plotly_white',
                  font_size=20,
                  yaxis_title="Execution Time (μs)",
                  xaxis_title="Implementation",
                  )

fig.update_traces(textposition='outside')

fig.show()
