package com.example.rustkotlindemo

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp

import com.example.rustkotlindemo.ui.theme.RustKotlinDemoTheme
import uniffi.SharedLib.add
import uniffi.SharedLib.index
import uniffi.SharedLib.tokenize

class MainActivity : ComponentActivity() {
    init {
        System.loadLibrary("sharedlib")
    }
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            RustKotlinDemoTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    ContentViewSimple()
                }
            }
        }

    }
}


@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ContentViewSimple() {
    // simple add op only calculate by adding a and b
    val valueA = remember { mutableStateOf(0) }
    val valueB = remember { mutableStateOf(0) }
    val valueString = remember { mutableStateOf("") }

    Column(
        modifier = Modifier
            .padding(16.dp)
            .fillMaxWidth()
    ) {
        Row(
            modifier = Modifier
                .padding(vertical = 8.dp)
                .fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(text = "a:")


            TextField(value = valueA.value.toString(), onValueChange = { value ->
                valueA.value = value.toIntOrNull() ?: 0
            })
        }
        Row(
            modifier = Modifier
                .padding(vertical = 8.dp)
                .fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(text = "b:")
            TextField(
                value = valueB.value.toString(),
                onValueChange = { value ->
                    valueB.value = value.toIntOrNull() ?: 0
                }
            )
        }
        Text(text = "a + b = ${addStatic(valueA.value, valueB.value)}")

        // add space
        Spacer(modifier = Modifier.padding(8.dp))
        Row(
            modifier = Modifier
                .padding(vertical = 8.dp)
                .fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text(text = "String:")
            TextField(
                value = valueString.value,
                onValueChange = { value ->
                    valueString.value = value
                },
            )
        }

        Text(text = "tokenized = ${tokenized(valueString.value)}")
        Text(text = "tokenized = ${tokenize(valueString.value)}")

    }

}

fun tokenized(value: String): String {
    return index(value.length.toULong(), value)
//    return 1.toString()
}

fun addStatic(a: Int, b: Int): Int {
//    return add(a, b)
    return a + b
}
@Preview(showBackground = true)
@Composable
fun ContentViewSimplePreview() {
    RustKotlinDemoTheme {
        ContentViewSimple()
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    RustKotlinDemoTheme {
        Greeting("Android")
    }
}
